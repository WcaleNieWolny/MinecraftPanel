#[macro_use] extern crate rocket;

mod server_process;
mod config;
mod minecraft_routes;
mod auth;

use crate::config::ServerConfig;
use rocket::{http::Method, fs::FileServer};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::fs::relative;

use crate::{auth::{auth_routes}};

#[rocket::main]
async fn main() -> anyhow::Result<()>{

    let config = ServerConfig::new()?;
    let config_clone = config.clone();

    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    let rocket = rocket::build()
    .manage(config)
    .attach(server_process::stage().await)
    .attach(minecraft_routes::stage())
    .attach(minecraft_routes::shutdown_hook())
    .attach(auth_routes::stage())
    .attach(cors.to_cors().unwrap())
    .manage(cors.to_cors().unwrap())
    .mount("/", rocket_cors::catch_all_options_routes());

    //Please provide a better solution. No idea what I just did
    if config_clone.serve_backend {
        let rocket = rocket.mount("/public", FileServer::from(relative!("static")));
        let _ = rocket.launch().await;
    }else{
        let _ = rocket.launch().await;
    }

    Ok(())
}