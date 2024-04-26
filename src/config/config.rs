use std::{fs, path::Path};

use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

use super::db_config::DbConfig;
use crate::common::error::ApplicationError;
use crate::response::result::Result;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub db: DbConfig,
}

impl Config {
    pub fn load(file_path: impl AsRef<Path>) -> Result<Self> {
        let config = fs::read_to_string(file_path.as_ref()).map_err(|_| ApplicationError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "读取配置文件出错".to_string(),
        })?;
        serde_yaml::from_str(&config).map_err(|_| ApplicationError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "解析配置文件出错".to_string(),
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::load("config/config.yml").unwrap()
    }
}
#[cfg(test)]
mod tests {
    use crate::dao::{self, DB};

    use super::*;
    #[test]
    fn confog_load() {
        let config = Config::load("config/config.yml").unwrap();
        let r = dao::DB.get_rb_ref();
        println!("rbatis:{}",r.get_pool().is_ok())
    }
}
