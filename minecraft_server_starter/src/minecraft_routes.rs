use std::sync::Arc;

use rocket::Shutdown;
use rocket::response::stream::{EventStream, Event};
use rocket::{State, fairing::AdHoc};
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use serde_json::{json};
use tokio::sync::watch::Receiver;
use tokio_stream::StreamExt;
use rocket::tokio::select;


use crate::auth::auth_state::AuthState;
use crate::server_process::ServerProcess;

#[get("/list_players")]
async fn list_players(process: &State<Arc<ServerProcess>>, _auth_state: AuthState) ->  rocket::serde::json::Value{
    let list_players = process.list_players().await;
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
    mut shutdown: Shutdown,
    _auth_state: AuthState
) -> EventStream![] {
    let stdout_rx = stdout_rx.inner().clone();

    EventStream! {
        let mut watch_stream = tokio_stream::wrappers::WatchStream::new(stdout_rx).map(|data| {
            Event::json(&data)
        });

        loop {
            select! {
                msg = watch_stream.next() => {
                    match msg {
                        Some(it) => {
                            yield it;
                        },
                        None => break,
                    };  
                },
                _ = &mut shutdown => {
                    yield Event::json(&"Server is shutting down!");
                    break;
                }
            };
        }
    }
}

#[post("/execute_cmd", format = "json", data = "<message>")]
async fn execute_cmd(
    message: Json<CommandPost>, 
    process: &State<Arc<ServerProcess>>,
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

    println!("$ {}", cmd);

    process.write_to_stdin(cmd);

    Some(())
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Server Routes", |rocket| async move{

        rocket
            .mount("/api", routes![execute_cmd, list_players, console])
    })
}

pub fn shutdown_hook() -> AdHoc {
    AdHoc::on_shutdown("shutdown hook!", |rocket| Box::pin(async move {
        let server_process: &Arc<ServerProcess> = rocket.state().unwrap();
        println!("WHOLE BACKEND IS SHUTING DOWN!!! {}", Arc::strong_count(server_process));
        server_process.await_shutdown().await;
        println!("SHUTDOWN COMPLEATED")
    }))
}