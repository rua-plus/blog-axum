#![allow(dead_code)]

use config::Config;
use serde::Deserialize;
use std::path::PathBuf;
use tracing::debug;

#[derive(Debug, Deserialize, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub postgresql: PostgresConfig,
    pub jwt: JwtConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config_file = match std::env::var("CONFIG_FILE") {
            Ok(path) => PathBuf::from(path),
            Err(_) => PathBuf::from("config.toml"),
        };
        debug!("Config file path: {:?}", config_file);

        let mut config =
            Config::builder().add_source(config::File::with_name("config.toml").required(false));

        // 加载用户配置文件（如果存在）
        if config_file.exists() {
            config = config.add_source(config::File::from(config_file).required(false));
        }

        // 加载环境变量覆盖
        // config = config.add_source(
        //     config::Environment::with_prefix("APP")
        //         .prefix_separator("_")
        //         .separator("__"),
        // );

        config.build()?.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = AppConfig::load();
        assert!(config.is_ok());
        let config = config.unwrap();

        // 验证PostgreSQL配置
        assert!(!config.postgresql.host.is_empty());
        assert!(config.postgresql.port > 0);
        assert!(!config.postgresql.user.is_empty());
        assert!(!config.postgresql.database.is_empty());

        // 验证JWT配置
        assert!(!config.jwt.secret.is_empty());
        assert!(!config.jwt.expires_in.is_empty());
    }
}
