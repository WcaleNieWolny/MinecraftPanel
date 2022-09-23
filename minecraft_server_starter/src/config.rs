use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerConfig{
    pub version: String,
    pub mysql_string: &'static str
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            version:  "paper-1.18.2".to_string(),
            mysql_string: "mysql://root:my_secret_password@127.0.0.1:6033/panel_users"
        }
    }
}