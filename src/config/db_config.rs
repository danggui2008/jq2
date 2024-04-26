use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    #[serde(default = "default_pool_size")]
    pub max_connections: u32,
}
fn default_pool_size() -> u32 {
    32
}

impl DbConfig {

    pub fn url(&self) -> String {
        format!("mysql://{}:{}@{}:{}/{}", &self.user,&self.password,&self.host,&self.port,&self.dbname)
    }
}
