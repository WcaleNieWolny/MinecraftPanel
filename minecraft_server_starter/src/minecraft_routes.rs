use std::ops::Deref;
use std::sync::Arc;

use rocket::{State, fairing::AdHoc};
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use serde_json::{json};
use tokio::sync::Mutex;

use crate::server_process::{ServerProcess};

#[get("/last_std")]
async fn last_std(process: &State<Arc<Mutex<ServerProcess>>>) ->  rocket::serde::json::Value{
    let last_std = process.lock().await.last_stdout();
    json!({ "last_std": last_std })
}

#[get("/list_players")]
async fn list_players(process: &State<Arc<Mutex<ServerProcess>>>) ->  rocket::serde::json::Value{
    let list_players = process.lock().await.list_players().await;
    json!({ "last_std": list_players })
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CommandPost {
    command: String
}

#[post("/execute_cmd", format = "json", data = "<message>")]
async fn execute_cmd(message: Json<CommandPost>, process: &State<Arc<Mutex<ServerProcess>>>) -> Option<()> {
    let mut cmd = message.command.clone();
    let cmd = match cmd.starts_with("/") {
        true => {
            cmd.remove(0);
            cmd
        },
        false => cmd
    };

    println!("CMD: {}", cmd);

    process.lock().await.write_to_stdin(cmd);

    Some(())
}

pub fn stage(server_process: Arc<tokio::sync::Mutex<ServerProcess>>) -> AdHoc {
    AdHoc::on_ignite("Server Process", |rocket| async move{
        rocket.manage(server_process)
            .mount("/api", routes![last_std, execute_cmd, list_players])
    })
}