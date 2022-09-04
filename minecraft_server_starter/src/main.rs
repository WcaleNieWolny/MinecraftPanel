#[macro_use] extern crate rocket;

mod server_process;
mod config;
mod minecraft_routes;

use std::{process::Stdio, env, path::PathBuf, io::{BufRead}, sync::Arc, thread};
use config::ServerConfig;
use tokio::{sync::Mutex, fs::File, io::{AsyncWriteExt, AsyncReadExt}, process::Command};

use crate::server_process::ServerProcess; // 0.2.4, features = ["full"]

#[rocket::main]
async fn main() -> anyhow::Result<()>{

    let config = get_config().await?;

    let server_jar_path = prepere_server_jar(&config).await?;

    println!("VER: {}", config.version);

    let server_jar_path_str = server_jar_path.to_str().expect("Couldn't format server jar path to string");
    let server_jar_pwd_path = server_jar_path.parent().expect("Coudln't find parrent path for server jar");

    let cmd = Command::new("java")
        .arg("-jar")
        .arg(server_jar_path_str)
        .current_dir(server_jar_pwd_path)
        .stdout(Stdio::piped()) // Can do the same for stderr
        .stdin(Stdio::piped())
        .spawn()
        .expect("cannot spawn");

    let server_process = Arc::new(Mutex::new(ServerProcess::new(cmd).await));

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

    let _ = rocket::build().attach(minecraft_routes::stage(server_process)).launch().await;

    Ok(())
}

async fn get_config() -> anyhow::Result<ServerConfig>{
    let mut path = env::current_dir().unwrap();
    path.push("config.toml");

    let mut config_file = File::open(path.clone()).await?;

    if(config_file.metadata().await.unwrap().len() == 0){
        let config = ServerConfig{
            version: "paper-1.18.2".to_string()
        };

        let mut config_file = File::create(path).await?;
        config_file.write(toml::to_string(&config).unwrap().as_bytes()).await?;

        return Ok(config)
    }

    let mut config_contents = vec![];
    config_file.read_to_end(&mut config_contents).await?;
    let config_contents = String::from_utf8(config_contents)?;

    let config: ServerConfig = toml::from_str(&config_contents)?;

    Ok(config)
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