use std::net::SocketAddr;

use anyhow::{bail, Context};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Dev,
    Prod,
}

impl Mode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Prod => "prod",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub listen: SocketAddr,
    pub mode: Mode,
    pub version: String,
    pub api_prefix: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "3002".into())
            .parse()
            .context("invalid PORT (expected 0-65535)")?;

        let mode = match std::env::var("APP_MODE").as_deref() {
            Ok(v) => match v.to_ascii_lowercase().as_str() {
                "dev" | "development" => Mode::Dev,
                "prod" | "production" => Mode::Prod,
                other => bail!("APP_MODE must be 'dev' or 'prod', got '{other}'"),
            },
            Err(_) => Mode::Dev,
        };

        let version = match mode {
            Mode::Dev => format!("{}-dev", env!("GIT_HASH")),
            Mode::Prod => std::env::var("APP_VERSION")
                .context("APP_VERSION required when APP_MODE=prod")?,
        };

        let api_prefix = format!("v{version}");

        let database_url = std::env::var("DATABASE_URL")
            .context("DATABASE_URL required")?;

        Ok(Self {
            listen: SocketAddr::from(([0, 0, 0, 0], port)),
            mode,
            version,
            api_prefix,
            database_url,
        })
    }

    #[must_use]
    pub fn api_url_prefix(&self) -> String {
        format!("/api/{}", self.api_prefix)
    }
}
