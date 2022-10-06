use std::sync::Arc;

use argon2::{Argon2, Params, PasswordHash, PasswordVerifier};
use rand::distributions::{Alphanumeric, DistString};
use rocket::State;
use rocket::http::{CookieJar, Cookie, SameSite, Status};
use rocket::{fairing::AdHoc};
use rocket::serde::{Deserialize, json::Json, json::json};
use tokio::sync::{RwLock, Mutex};

use crate::auth::auth_state::AuthState;
use crate::config::{ServerConfig};

use super::database::{self, Connection};
use super::models::User;

#[derive(Debug, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}
//POST /auth/authenticate_user application/json
//POST /api/auth/authenticate_user application/json
//Admin pwd: jz6u8s0ea24HcMK
#[post("/authenticate_user", format="json", data = "<message>")]
async fn authenticate_user(
    jar: &CookieJar<'_>, 
    message: Json<LoginForm>, 
    argon: &State<Argon2<'_>>, 
    mut connection: Connection,
    cache: &State<Arc<RwLock<Vec<AuthState>>>>
) ->  Result<rocket::serde::json::Value, (Status, Option<rocket::serde::json::Value>)> {

    let password = &message.password;
    let username = &message.username;

    let user = User::read_by_username(username, &mut connection);

    match user {
        Ok(user) => {
            let parsed_hash = match PasswordHash::new(&user.password) {
                Ok(val) => val,
                Err(_) => return Err((Status::InternalServerError, None)),
            };

            if argon.inner().verify_password(password.as_bytes(), &parsed_hash).is_ok(){
                println!("PWD MATCH!");

                let auth_state = match AuthState::new(user) {
                    Ok(auth_state) => auth_state,
                    Err(_) => return Err((Status::InternalServerError, None)),
                };

                let auth_id = auth_state.put_in_cache(cache).await;

                jar.add_private(
                    Cookie::build("user_id", auth_id.to_string())
                        .expires(None)
                        .same_site(SameSite::None)
                        .finish()
                );
              
                Ok (json!({ "status": "OK" }))
            }else{
                Err((Status::BadRequest, Some(json!({ "error": "Invalid credentials" }))))
            }
        },
        Err(_) => {
            Err((Status::BadRequest, Some(json!({ "error": "Invalid credentials" }))))
        }
    }
    //ws_auth_vec: Arc<RwLock<Vec<String>>>
}

#[get("/request_console")]
async fn request_console(
    auth_state: AuthState
) ->  Result<rocket::serde::json::Value, (Status, Option<rocket::serde::json::Value>)> {
    Ok(json!({ "hash": auth_state.web_socket_auth_token }))
}

pub fn stage(config: ServerConfig) -> AdHoc {

    AdHoc::on_ignite("Auth Stage", |rocket| async {

        let argon = Argon2::new(
            argon2::Algorithm::Argon2id,
             argon2::Version::V0x13,
              Params::new(
                163864u32,
                2,
                1,
                None 
            ).unwrap()
        );

        rocket.mount("/auth", routes![authenticate_user, request_console])
            .manage(database::connect(config, &argon))
            .manage(argon)
    })
}