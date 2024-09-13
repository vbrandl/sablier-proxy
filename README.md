# Sablier TCP Proxy

![build](https://github.com/vbrandl/sablier-proxy/actions/workflows/rust.yml/badge.svg)

A simple TCP proxy that connects to [Sablier](https://github.com/acouvreur/sablier) on each incoming connection.
The backend will be started and stopped according to the Sablier configuration.

Every time a new client connect, the Sablier blocking endpoint is called.
As long as there is at least one active connection, a background task pings the Sablier endpoint to prevent the services from shutting down if there are only long running connections.

Since this proxy is not protocol aware and not limited to HTTP, the dynamic Sablier configuration doesn't make sense. Therefore only the blocking configuration is implemented.

## Usage and Configuration

Replace your Caddy, Traefik or Nginx reverse proxy with `ghcr.io/vbrandl/sablier-proxy` and configure the proxy service.

The proxy is configured using environment variables and matches the Sablier configuration for other reverse proxies.

| Variable | Default Value | Description
| --- | --- | --- |
| `LISTEN` | | IP and port to listen on |
| `UPSTREAM` | | Upstream IP and port |
| `SABLIER_URL` | `http://sablier:10000` | URL of the Sablier service. Must not end in `/` |
| `NAMES` | | |
| `GROUP` | | |
| `SESSION_DURATION` | | Duration until the backend will be shut down |
| `BLOCKING_TIMEOUT` | | |
| `PING_INTERVAL_SEC` | `10` | Interval in which the Sablier endpoint is pinged if there is at least one active connection |
| `UPSTREAM_RETRIES` | `5` | When the backend gets started, this number controls how often the Sablier endpoint is checked before the connection will be dropped |
| `UPSTREAM_RETRY_DURATION_MS` | `200` | Duration between `UPSTREAM_RETRIES` in ms |

### Example

The following example will shut down the `echo` service after 10 seconds of inactivity

```yaml
services:
  proxy:
    image: ghcr.io/vbrandl/sablier-proxy:latest
    ports:
      - '8080:8080'
    environment:
      - LISTEN="0.0.0.0:8080"
      - UPSTREAM="echo:80"
      - GROUP="demo"
      # shutdown echo server after 10s
      - SESSION_DURATION="10s"
      # block 2s when waiting for the echo server to start
      - BLOCKING_TIMEOUT="2s"

  echo:
    image: cjimti/go-echo
    environment:
      - TCP_PORT=80
    labels:
      - sablier.enable=true
      - sablier.group=demo

  sablier:
    image: acouvreur/sablier:1.7.0
    command:
        - start
        - --provider.name=docker
    volumes:
      - '/var/run/docker.sock:/var/run/docker.sock'
```

## License

`sablier-proxy` is licensed under the MIT License ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).
