use argon2::{Argon2, Params, PasswordHash, PasswordVerifier};
use rocket::State;
use rocket::http::{CookieJar, Cookie, SameSite, Status};
use rocket::{fairing::AdHoc};
use rocket::serde::{Deserialize, json::Json, json::json};

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
//Admin pwd: rnTLt972RdI0NjK
#[post("/authenticate_user", format="json", data = "<message>")]
async fn authenticate_user(
    jar: &CookieJar<'_>, 
    message: Json<LoginForm>, 
    argon: &State<Argon2<'_>>, 
    mut connection: Connection
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

                if cfg!(feature = "debug"){
                    info!("PWD MATCH!");
                }

                let auth_id = match AuthState::create_session(&user, &mut connection) {
                    Ok(val) => val,
                    Err(_) => return Err((Status::InternalServerError, None)),
                };

                jar.add_private(
                    Cookie::build("user_id", auth_id)
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
}

#[get("/logout")]
async fn logout(
    auth_state: AuthState,
    mut connection: Connection
) -> Status {
    return match auth_state.logout(&mut connection).await {
        Ok(_) => Status::NoContent,
        Err(_) => Status::InternalServerError,
    };
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth Stage", |rocket| async {
        let config = rocket.state::<ServerConfig>().unwrap().clone();

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
        
        rocket.mount("/auth", routes![authenticate_user, logout])
            .manage(database::connect(config, &argon))
            .manage(argon)
    })
}