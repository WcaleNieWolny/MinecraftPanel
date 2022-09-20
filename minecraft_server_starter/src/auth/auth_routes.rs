use rocket::http::{CookieJar, Cookie, SameSite};
use rocket::{fairing::AdHoc};
use rocket::serde::{Deserialize, json::Json, json::json};

#[derive(Debug, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}
//POST /auth/authenticate_user application/json
//POST /api/auth/authenticate_user application/json
#[post("/authenticate_user", format="json", data = "<message>")]
fn authenticate_user(jar: &CookieJar<'_>, message: Json<LoginForm>) -> rocket::serde::json::Value {

    jar.add_private(Cookie::new("user_id", 1.to_string()));

    println!("{:?}", message);

    json!({ "status": "OK" })
}

#[get("/test")]
fn test(jar: &CookieJar<'_>){
    //Fix get_private returning NONE
    let val = jar.get_private("user_id");

    println!("TEST AA!");

    for c in jar.iter() {
        println!("Name: {:?}, Value: {:?}", c.name(), c.value());
        println!("HL {:?}", jar.get_private(c.name()))
//TEST AA!
//Name: "user_id", Value: "S5bTvL2/WmZaEjkqiPLRvEcmb+Ev7Hx5AwjdOxY="
//HL None
    }

    if val.is_some(){
        println!("{}", val.unwrap())
    }else {
        println!("NULL!")
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth Stage", |rocket| async {
        rocket.mount("/auth", routes![authenticate_user, test])
    })
}