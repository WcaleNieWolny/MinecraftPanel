use std::sync::Arc;

use rocket::response::stream::{EventStream, Event};
use rocket::{State, fairing::AdHoc};
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use serde_json::{json};
use tokio::sync::Mutex;
use tokio::sync::watch::Receiver;
use tokio_stream::StreamExt;

use crate::auth::auth_state::AuthState;
use crate::server_process::{ServerProcess};

#[get("/last_std")]
async fn last_std(process: &State<Arc<Mutex<ServerProcess>>>, _auth_state: AuthState) ->  rocket::serde::json::Value{
    let last_std = process.lock().await.last_stdout();
    json!({ "last_std": last_std })
}

#[get("/list_players")]
async fn list_players(process: &State<Arc<Mutex<ServerProcess>>>, _auth_state: AuthState) ->  rocket::serde::json::Value{
    let list_players = process.lock().await.list_players().await;
    let size = list_players.len();
    json!({ "list_players": list_players, "size": size })
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CommandPost {
    command: String
}

#[get("/console")]
async fn console(
    stdout_rx: &State<Receiver<String>>,
    _auth_state: AuthState
) -> EventStream![] {

    let stdout_rx = stdout_rx.inner().clone();

    EventStream! {
        let mut watch_stream = tokio_stream::wrappers::WatchStream::new(stdout_rx).map(|data| {
            Event::json(&data)
        });
        loop {
            match watch_stream.next().await {
                Some(it) => {
                    yield it;
                },
                None => break,
            };
        }
    }
}

#[post("/execute_cmd", format = "json", data = "<message>")]
async fn execute_cmd(
    message: Json<CommandPost>, 
    process: &State<Arc<Mutex<ServerProcess>>>,
    _auth_state: AuthState
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
            .mount("/api", routes![last_std, execute_cmd, list_players, console])
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