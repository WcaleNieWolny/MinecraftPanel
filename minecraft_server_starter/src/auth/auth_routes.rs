use rocket::http::{CookieJar, Cookie, SameSite};
use rocket::{fairing::AdHoc};
use rocket::serde::{Deserialize, json::Json, json::json};

use crate::config::{self, ServerConfig};

use super::database::{self, Connection};
use super::models::User;

#[derive(Debug, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}
//POST /auth/authenticate_user application/json
//POST /api/auth/authenticate_user application/json
#[post("/authenticate_user", format="json", data = "<message>")]
fn authenticate_user(jar: &CookieJar<'_>, message: Json<LoginForm>) -> rocket::serde::json::Value {

    jar.add_private(
        Cookie::build("user_id", 1.to_string())
            .same_site(SameSite::None)
            .finish()
    );

    println!("{:?}", message);

    json!({ "status": "OK" })
}

#[get("/get")]
fn get(jar: &CookieJar<'_>, mut connection: Connection){
    //Fix get_private returning NONE
    let val = jar.get_private("user_id");

    println!("TEST AA!");

    for c in jar.iter() {
        println!("Name: {:?}, Value: {:?}", c.name(), c.value());
        println!("HL {:?}", jar.get_private(c.name()))
    }

    let users = User::read_all(&mut connection);

    for u in users.iter(){
        println!("U: {:?}", u)
    }

    if val.is_some(){
        println!("{}", val.unwrap())
    }else {
        println!("NULL!")
    }
}

pub fn stage(config: ServerConfig) -> AdHoc {
    AdHoc::on_ignite("Auth Stage", |rocket| async {
        rocket.mount("/auth", routes![authenticate_user, get])
            .manage(database::connect(config))
    })
}