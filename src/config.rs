use std::time::Duration;

use anyhow::{anyhow, Result};
use figment::{providers::Env, Figment};
use reqwest::{Client, Response};
use serde::Deserialize;
use tokio::time::sleep;

const API_PATH: &str = "/api/strategies/blocking";

fn default_url() -> String {
    "http://sablier:10000".to_string()
}

fn default_ping_interval() -> u64 {
    10
}

fn default_upstream_retries() -> u8 {
    5
}

fn default_upstream_retry_duration_ms() -> u64 {
    200
}

#[derive(Deserialize)]
pub struct Config {
    pub listen: String,
    pub upstream: String,
    #[serde(default = "default_url")]
    sablier_url: String,
    names: Option<String>,
    group: Option<String>,
    session_duration: String,
    // only blocking makes sense for TCP
    blocking_timeout: Option<String>,
    #[serde(default = "default_ping_interval")]
    pub ping_interval_sec: u64,
    #[serde(default = "default_upstream_retries")]
    upstream_retries: u8,
    #[serde(default = "default_upstream_retry_duration_ms")]
    upstream_retry_duration_ms: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(Figment::new().merge(Env::prefixed("")).extract()?)
    }
}

impl Config {
    fn normalize_sablier_url(&self) -> &str {
        self.sablier_url
            .strip_suffix('/')
            .unwrap_or(&self.sablier_url)
    }

    pub async fn request(&self) -> Result<Response> {
        let client = Client::new();
        let url = format!("{}{API_PATH}", self.normalize_sablier_url());
        let mut request = client.get(url).query(&[
            ("session_duration", &self.session_duration)
        ]);
        if let Some(names) = &self.names {
            let names: Vec<_> = names.split(',').map(str::trim).collect();
            request = request.query(&[("names", &names)]);
        }
        if let Some(timeout) = &self.blocking_timeout {
            request = request.query(&[("timeout", &timeout)]);
        }
        if let Some(group) = &self.group {
            request = request.query(&[("group", &group)]);
        }
        Ok(request.send().await?)
    }

    pub async fn wait_for_upstream(&self) -> Result<()> {
        let duration = Duration::from_millis(self.upstream_retry_duration_ms);
        for _ in 0..self.upstream_retries {
            let response = self.request().await?;
            if response
                .headers()
                .get("X-Sablier-Session-Status")
                .is_some_and(|v| v == "ready")
            {
                return Ok(());
            }
            sleep(duration).await;
        }
        Err(anyhow!("upstream hasn't started yet"))
    }
}
