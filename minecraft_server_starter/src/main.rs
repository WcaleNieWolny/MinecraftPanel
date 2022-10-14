#[macro_use] extern crate rocket;

mod server_process;
mod config;
mod minecraft_routes;
mod auth;

use std::{process::Stdio, env, path::PathBuf, io::{BufRead}, sync::Arc, thread, collections::HashMap};
use crate::config::ServerConfig;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio::{sync::{Mutex, RwLock}, io::AsyncReadExt, process::Command};

use crate::{server_process::ServerProcess, auth::{auth_routes, auth_state::{self, AuthState}}}; // 0.2.4, features = ["full"]

#[rocket::main]
async fn main() -> anyhow::Result<()>{

    let config = ServerConfig::new()?;

    let server_jar_path = prepere_server_jar(&config).await?;

    println!("VER: {}", config.version);

    let server_jar_path_str = server_jar_path.to_str().expect("Couldn't format server jar path to string");
    let server_jar_pwd_path = server_jar_path.parent().expect("Coudln't find parrent path for server jar");

    let cmd = Command::new("java")
        .arg("-Dterminal.jline=false")
        .arg("-Dterminal.ansi=true")
        .arg("-jar")
        .arg(server_jar_path_str)
        .arg("nogui")
        .current_dir(server_jar_pwd_path)
        .stdout(Stdio::piped()) // Can do the same for stderr
        .stdin(Stdio::piped())
        .spawn()
        .expect("cannot spawn");

    let server_process = ServerProcess::new(cmd).await;

    let stdout_rx = server_process.stdout_rx.clone(); 

    let server_process = Arc::new(Mutex::new(server_process));

    let server_process_clone = server_process.clone();
    thread::spawn(move || {
        let stdin = std::io::stdin();

        loop{
            for line in stdin.lock().lines() {
                let line = line.unwrap();
                server_process_clone.blocking_lock().write_to_stdin(line);
            }
        }
    });

    let auth_vec = Arc::new(RwLock::new(HashMap::<String, AuthState>::new()));

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
    .attach(auth_state::stage(auth_vec))
    .attach(minecraft_routes::stage(server_process.clone()))
    .attach(minecraft_routes::shutdown_hook(server_process))
    .attach(auth_routes::stage(config.clone()))
    .attach(cors.to_cors().unwrap())
    .manage(cors.to_cors().unwrap())
    .manage(stdout_rx)
    .mount("/", rocket_cors::catch_all_options_routes())
    .launch().await;

    Ok(())
}

async fn prepere_server_jar(servrer_config: &ServerConfig) -> anyhow::Result<PathBuf>{
    let mut path = env::current_dir().unwrap();
    path.push("run");

    tokio::fs::create_dir_all(path).await?;

    let mut path = env::current_dir().unwrap();
    path.push("run/srv.jar");

    println!("EXIST: {}", path.exists());

    let version = servrer_config.version.replace("paper-", "");
    let build_id = make_paper_build_id_request(&version).await?;

    download_paper_jar(&version, build_id, &path).await?;

    Ok(path)
}

async fn make_paper_build_id_request(version: &String) -> anyhow::Result<i64>{
    let url = format!("https://api.papermc.io/v2/projects/paper/versions/{}", version);
    let builds_response = reqwest::Client::new()
    .get(url).send().await?;

    let build_json = builds_response.json::<serde_json::Value>().await?;

    let builds = &build_json["builds"];
    let builds = builds.as_array().expect("Coudln't get builds when fetching paper builds");

    let latest_build = &builds[builds.len() - 1];
    let latest_build = latest_build.as_i64().expect("Couln't convert version to i64");

    println!("Fetched latest paper build as {}", latest_build);

    Ok(latest_build)
}

async fn download_paper_jar(version: &String, build_id: i64, path: &PathBuf) -> anyhow::Result<()>{
    let url = format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/paper-{}-{}.jar", version, build_id, version, build_id);

    let builds_response = reqwest::Client::new()
    .get(url).send().await?;
    let builds_response = builds_response.bytes().await?;

    let mut data = builds_response.take(builds_response.len() as u64);

    //let a = builds_response.iter().collect();
    //https://stackoverflow.com/questions/44438059/how-to-convert-from-stdiobytes-to-u8

    let mut file = tokio::fs::File::create(path).await?;

    tokio::io::copy(&mut data, &mut file).await?;
    //tokio::io::copy(&mut builds_response, file).await?;

    Ok(())
}