use std::thread::Thread;
use std::time::Duration;

use argon2::{Argon2, Params, PasswordHash, PasswordVerifier};
use rand::Rng;
use rocket::State;
use rocket::http::{CookieJar, Cookie, SameSite};
use rocket::response::status::{self, BadRequest};
use rocket::{fairing::AdHoc};
use rocket::serde::{Deserialize, json::Json, json::json};
use tokio::time::Sleep;

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
fn authenticate_user(jar: &CookieJar<'_>, message: Json<LoginForm>, argon: &State<Argon2>, mut connection: Connection) ->  Result<rocket::serde::json::Value, status::BadRequest<String>> {

    jar.add_private(
        Cookie::build("user_id", 1.to_string())
            .same_site(SameSite::None)
            .finish()
    );

    let password = &message.password;
    let username = &message.username;

    let user = User::read_by_username(username, &mut connection);

    match user {
        Ok(user) => {
            let parsed_hash = match PasswordHash::new(&user.password) {
                Ok(val) => val,
                Err(_) => return Err(BadRequest(Some("Internal error".to_string()))),
            };

            if argon.inner().verify_password(password.as_bytes(), &parsed_hash).is_ok(){
                println!("PWD MATCH!");
                Ok (json!({ "status": "OK" }))
            }else{
                Err(BadRequest(Some("Invalid credentials".to_string())))
            }
        },
        Err(_) => {
            Err(BadRequest(Some("Invalid credentials".to_string())))
        }
    }
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

        rocket.mount("/auth", routes![authenticate_user])
            .manage(database::connect(config, &argon))
            .manage(argon)
    })
}