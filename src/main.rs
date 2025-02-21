//! A TCP proxy server that works with [sablier](https://github.com/acouvreur/sablier).
//!
//! When a new connection is handled, a request is sent to `sablier` to start the backend services.
//! Also while there are active connections, `sablier` is pinged in regular intervals, so the
//! backend won't be stopped if there is only one long running connection.
//!
//! The proxy logic is strongly inspired by <https://github.com/mqudsi/tcpproxy>.
mod config;

use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::config::Config;

use anyhow::Result;
use futures::FutureExt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::broadcast,
    time::sleep,
};
use tracing::{error, info, trace};

fn init_logging() {
    tracing_subscriber::fmt().pretty().init();
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let config = Config::load()?;
    proxy(config).await?;
    Ok(())
}

async fn proxy(config: Config) -> Result<()> {
    let listener = TcpListener::bind(&config.listen).await?;
    let config = Arc::new(config);
    let active_connections = Arc::new(AtomicU64::new(0));

    tokio::spawn(ping_while_active_connection(
        config.clone(),
        active_connections.clone(),
    ));

    loop {
        let (mut client, client_addr) = listener.accept().await?;
        info!(client = %client_addr, "new connection");
        let config = config.clone();
        let active_connections = active_connections.clone();
        tokio::spawn(async move {
            if let Err(err) = config.wait_for_upstream().await {
                error!(error = %err, "error requesting upstream status");
                return;
            }
            let mut remote = match TcpStream::connect(&config.upstream).await {
                Ok(remote) => remote,
                Err(err) => {
                    error!(client=%client_addr, error = %err, "cannot connect to upstream");
                    return;
                }
            };
            active_connections.fetch_add(1, Ordering::Relaxed);
            let (mut client_read, mut client_write) = client.split();
            let (mut remote_read, mut remote_write) = remote.split();

            let (cancel, _) = broadcast::channel::<()>(1);
            let (remote_copied, client_copied) = tokio::join! {
                copy_with_abort(&mut remote_read, &mut client_write, cancel.subscribe())
                    .then(|r| { let _ = cancel.send(()); async {r}}),
                copy_with_abort(&mut client_read, &mut remote_write, cancel.subscribe())
                    .then(|r| { let _ = cancel.send(()); async {r}}),
            };

            active_connections.fetch_sub(1, Ordering::Relaxed);

            match client_copied {
                Ok(count) => {
                    trace!(size = count, client = %client_addr, "Sent to upstream");
                }
                Err(err) => {
                    error!(client = %client_addr, error = %err, "Error sending to upstream");
                }
            }

            match remote_copied {
                Ok(count) => {
                    trace!(size = count, client = %client_addr, "Sent to client");
                }
                Err(err) => {
                    error!(client = %client_addr, error = %err, "Error sending to client");
                }
            }
            info!(client = %client_addr, "connection closed");
        });
    }
}

// Two instances of this function are spawned for each half of the connection: client-to-server,
// server-to-client. We can't use tokio::io::copy() instead (no matter how convenient it might
// be) because it doesn't give us a way to correlate the lifetimes of the two tcp read/write
// loops: even after the client disconnects, tokio would keep the upstream connection to the
// server alive until the connection's max client idle timeout is reached.
async fn copy_with_abort<R, W>(
    read: &mut R,
    write: &mut W,
    mut abort: broadcast::Receiver<()>,
) -> tokio::io::Result<usize>
where
    R: tokio::io::AsyncRead + Unpin,
    W: tokio::io::AsyncWrite + Unpin,
{
    const BUF_SIZE: usize = 0xffff;

    let mut copied = 0;
    let mut buf = vec![0u8; BUF_SIZE].into_boxed_slice();
    loop {
        let bytes_read;
        tokio::select! {
            biased;

            result = read.read(&mut buf) => {
                use std::io::ErrorKind::{ConnectionReset, ConnectionAborted};
                bytes_read = result.or_else(|e| match e.kind() {
                    // Consider these to be part of the proxy life, not errors
                    ConnectionReset | ConnectionAborted => Ok(0),
                    _ => Err(e)
                })?;
            },
            _ = abort.recv() => {
                break;
            }
        }

        if bytes_read == 0 {
            break;
        }

        // While we ignore some read errors above, any error writing data we've already read to
        // the other side is always treated as exceptional.
        write.write_all(&buf[0..bytes_read]).await?;
        copied += bytes_read;
    }

    Ok(copied)
}

/// Ping the sablier endpoint in regular intervals while there are active connections.
///
/// This is done to prevent the backend from shutting down in case there are only long running TCP
/// connections.
async fn ping_while_active_connection(config: Arc<Config>, active_connections: Arc<AtomicU64>) {
    info!("starting sablier ping task");
    let interval = Duration::from_secs(config.ping_interval_sec);
    loop {
        let sleep = sleep(interval);
        tokio::pin!(sleep);
        tokio::select! {
            biased;

            () = &mut sleep => {
                let count = active_connections.load(Ordering::Relaxed);
                if count != 0 {
                    trace!(conns = count, "pinging sablier");
                    if let Err(err) = config.request().await {
                        error!(conns = count, error = %err, "error pinging sablier");
                    }
                } else {
                    trace!(conns = count, "no active connections. not pinging");
                }
            }
        }
    }
}
