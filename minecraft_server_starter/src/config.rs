use std::env;

use config::{Config, File, Environment};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerConfig{
    pub version: String,
    pub mysql_string: String,
    pub serve_frontend: bool
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            version: "paper-1.18.2".to_string(),
            mysql_string: "mysql://root:my_secret_password@127.0.0.1:6033/panel_users".to_string(),
            serve_frontend: true
        }
    }
}

impl ServerConfig {
    pub fn new() -> anyhow::Result<ServerConfig>{
        let mut path = env::current_dir().unwrap();
        path.push("config.toml");
        let path = match path.to_str() {
           Some(val) => val,
           None => return Err(anyhow::Error::msg("Couldn't parse path when generating config")) 
        };
    
        let config = Config::builder()
            .add_source(File::with_name(path))
            .add_source(
                Environment::with_prefix("PANEL")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(" "),
            )
            .build()
            .unwrap();
    
        let server_config: ServerConfig = config.try_deserialize().unwrap();
    
        println!("CONF: {:?}", server_config);
    
        Ok(server_config)
    }
}