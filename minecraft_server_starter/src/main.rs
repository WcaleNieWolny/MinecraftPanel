#[macro_use] extern crate rocket;

mod server_process;
mod config;
mod minecraft_routes;
mod auth;

use std::{sync::Arc, collections::HashMap};
use crate::config::ServerConfig;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio::sync::RwLock;

use crate::{auth::{auth_routes, auth_state::{self, AuthState}}}; // 0.2.4, features = ["full"]

#[rocket::main]
async fn main() -> anyhow::Result<()>{

    let config = ServerConfig::new()?;

    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    let _ = rocket::build()
    .manage(config)
    .attach(server_process::stage().await)
    .attach(auth_state::stage())
    .attach(minecraft_routes::stage())
    .attach(minecraft_routes::shutdown_hook())
    .attach(auth_routes::stage())
    .attach(cors.to_cors().unwrap())
    .manage(cors.to_cors().unwrap())
    .mount("/", rocket_cors::catch_all_options_routes())
    .launch().await;

    Ok(())
}