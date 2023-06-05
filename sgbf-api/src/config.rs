use std::net::IpAddr;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: Server,
    pub tracing: crate::tracing::TracingConfig
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("SGBF").separator("__"))
            .build()?;

        cfg.try_deserialize()
    }
}
