use std::sync::Arc;

use rocket::{State, fairing::AdHoc};
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use serde_json::{json};
use tokio::sync::Mutex;

use crate::auth::auth_state::{self, AuthState};
use crate::server_process::{ServerProcess};

#[get("/last_std")]
async fn last_std(process: &State<Arc<Mutex<ServerProcess>>>) ->  rocket::serde::json::Value{
    let last_std = process.lock().await.last_stdout();
    json!({ "last_std": last_std })
}

#[get("/list_players")]
async fn list_players(process: &State<Arc<Mutex<ServerProcess>>>) ->  rocket::serde::json::Value{
    let list_players = process.lock().await.list_players().await;
    let size = list_players.len();
    json!({ "list_players": list_players, "size": size })
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CommandPost {
    command: String
}

#[post("/execute_cmd", format = "json", data = "<message>")]
async fn execute_cmd(
    message: Json<CommandPost>, 
    process: &State<Arc<Mutex<ServerProcess>>>,
    auth_state: AuthState
) -> Option<()> {
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

pub fn shutdown_hook(server_process: Arc<tokio::sync::Mutex<ServerProcess>>) -> AdHoc {
    AdHoc::on_shutdown("shutdown hook!", |_| Box::pin(async move {
        println!("WHOLE BACKEND IS SHUTING DOWN!!!");
        let mut process = server_process.lock().await;
        process.await_shutdown().await;
        println!("SHUTDOWN COMPLEATED")
    }))
}