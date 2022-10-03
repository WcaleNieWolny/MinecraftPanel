use std::{time::{SystemTime, Duration}, sync::Arc};
use std::ops::{Deref};
use rocket::{http::{Status, CookieJar}, fairing::AdHoc};
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use rocket::outcome::{Outcome};
use rocket::outcome::try_outcome;
use tokio::sync::RwLock;

use super::models::User;

#[derive(Clone, Debug)]
pub enum UserType {
    NORMAL,
    ADMIN
}

#[derive(Clone, Debug)]
pub struct AuthState{
    creation_time: SystemTime,
    expire_time: Duration,
    pub username: String,
    pub user_type: UserType
}

impl AuthState{

    pub fn new(db_user: User) -> anyhow::Result<Self>{
        Ok(
            Self {
                creation_time: SystemTime::now(), 
                expire_time: Duration::from_secs(15 * 16), 
                username: db_user.username, 
                user_type: match db_user.user_type {
                    0 => UserType::NORMAL,
                    1 => UserType::ADMIN,
                    _ => return Err(anyhow::Error::msg("Invalid user type in the database"))
                }
            }
        )
    }

    pub async fn put_in_cache(self, cache: &State<RwLock<Vec<AuthState>>>) -> usize{
        let mut lock = cache.write().await;
        lock.push(self);
        return lock.len() - 1;
    }
}

pub fn stage() -> AdHoc {

    AdHoc::on_ignite("Auth StateStage", |rocket| async {

        let vec: RwLock<Vec<AuthState>> = RwLock::new(Vec::new());

        rocket.manage(vec)
    })
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthState {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) ->   request::Outcome<AuthState, Self::Error> {
        let user_id = req.cookies().get_private("user_id");

        match user_id {
            Some(user_id) => {
                let cache_vec: &State<RwLock<Vec<AuthState>>> = try_outcome!(req.guard::<&State<RwLock<Vec<AuthState>>>>().await);
                let user_id = match user_id.value().parse::<usize>() {
                    Ok(val) => val,
                    Err(_) => return Outcome::Failure((Status::InternalServerError, ())),
                };

                let cache_vec = cache_vec.read().await;

                return match cache_vec.get(user_id) {
                    Some(val) => return Outcome::Success(val.clone()),
                    None => Outcome::Failure((Status::Unauthorized, ())),
                };
            },
            None => {
                Outcome::Failure((Status::Unauthorized, ()))
            }
        }
    }
}