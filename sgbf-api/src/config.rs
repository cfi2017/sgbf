use std::net::IpAddr;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl Database {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CacheConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OneSignal {
    pub key: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: Server,
    pub cache: CacheConfig,
    pub database: Database,
    pub onesignal: OneSignal,
    pub tracing: crate::tracing::TracingConfig
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::File::with_name("config.override").required(false))
            .add_source(config::Environment::with_prefix("SGBF").separator("__"))
            .build()?;

        cfg.try_deserialize()
    }
}
