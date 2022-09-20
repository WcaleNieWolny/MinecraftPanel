use rocket::{fairing::AdHoc};
use rocket::serde::{Deserialize, json::Json, json::json};

#[derive(Debug, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/authenticate_user", format="json", data = "<message>")]
fn authenticate_user(message: Json<LoginForm>) -> rocket::serde::json::Value {

    println!("{:?}", message);

    json!({ "status": "OK" })
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth Stage", |rocket| async {
        rocket.mount("/auth", routes![authenticate_user])
    })
}