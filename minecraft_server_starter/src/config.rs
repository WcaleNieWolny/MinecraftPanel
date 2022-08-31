use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct ServerConfig{
    pub version: String
}