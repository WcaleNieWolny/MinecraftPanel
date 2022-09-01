mod server_process;
mod config;

use core::slice::SlicePattern;
use std::{process::Stdio, env, str::FromStr, path::PathBuf};
use config::ServerConfig;
use tokio::{process::Command, fs::{File}, io::{AsyncWriteExt, AsyncReadExt}};

use crate::server_process::ServerProcess; // 0.2.4, features = ["full"]

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let mut path = env::current_dir().unwrap();
    path.push("config.toml");

    let mut config_file = File::open(path.clone()).await?;

    let config_contents = if let 0 = config_file.metadata().await.unwrap().len() {
        let mut config_file = File::create(path).await?;
                let toml_str = 
    r#"
    version = "paper-1.18.2"
"#;

    config_file.write(toml_str.as_bytes()).await?;
    String::from_str(toml_str)?
    } else {
                let mut config_contents = vec![];
                config_file.read_to_end(&mut config_contents).await?;
                String::from_utf8(config_contents)?
            };

    let config: ServerConfig = toml::from_str(&config_contents)?;

    prepere_server_jar(&config).await?;

    println!("VER: {}", config.version);

    let cmd = Command::new("sh")
        .arg("/home/wolny/dev/MinecraftPanel/test.sh")
        .stdout(Stdio::piped()) // Can do the same for stderr
        .stdin(Stdio::piped())
        .spawn()
        .expect("cannot spawn");

    let mut server_process = ServerProcess::new(cmd).await;

    let a = server_process.read_from_stdout().await;

    println!("GOT A: {}", a);

    server_process.write_to_stdin("Michael".to_string());
    let c = server_process.read_from_stdout().await;

    println!("C: {}", c);

    Ok(())
}

async fn prepere_server_jar(servrer_config: &ServerConfig) -> anyhow::Result<()>{
    let mut path = env::current_dir().unwrap();
    path.push("run");

    tokio::fs::create_dir_all(path).await?;

    let mut path = env::current_dir().unwrap();
    path.push("run/srv.jar");

    println!("EXIST: {}", path.exists());

    let build_id = make_paper_build_id_request(servrer_config.version.replace("paper-", "")).await?;

    Ok(())
}

async fn make_paper_build_id_request(version: String) -> anyhow::Result<i64>{
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

async fn download_paper_jar(version: String, build_id: i64, path: PathBuf) -> anyhow::Result<()>{
    let url = format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/paper-{}-{}.jar", version, build_id, version, build_id);

    let builds_response = reqwest::Client::new()
    .get(url).send().await?;
    let builds_response = builds_response.bytes().await?;

    //let a = builds_response.iter().collect();
    //https://stackoverflow.com/questions/44438059/how-to-convert-from-stdiobytes-to-u8

    let file = tokio::fs::File::open(path).await?;

    file.write(builds_response.as_sl);

    Ok(())
}