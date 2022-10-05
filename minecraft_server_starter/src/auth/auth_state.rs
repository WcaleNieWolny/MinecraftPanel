use std::{time::{SystemTime, Duration}, sync::Arc, ops::AddAssign};
use rocket::{http::{Status}, fairing::AdHoc};
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use rocket::outcome::{Outcome};
use rocket::outcome::try_outcome;
use tokio::sync::{RwLock, Mutex};

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
                expire_time: Duration::from_secs(60 * 60), 
                username: db_user.username, 
                user_type: match db_user.user_type {
                    0 => UserType::NORMAL,
                    1 => UserType::ADMIN,
                    _ => return Err(anyhow::Error::msg("Invalid user type in the database"))
                }
            }
        )
    }

    pub async fn put_in_cache(self, cache: &State<Arc<RwLock<Vec<Mutex<AuthState>>>>>) -> usize{
        let mut lock = cache.write().await;
        lock.push(Mutex::new(self));
        return lock.len() - 1;
    }
}

pub fn stage() -> AdHoc {

    AdHoc::on_ignite("Auth StateStage", |rocket| async {

        let vec = Arc::new(RwLock::new(Vec::<Mutex<AuthState>>::new()));
        let vec_clone = vec.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10 * 60));

            loop {
                interval.tick().await;

                let mut write = vec_clone.write().await;

                let mut i = 0;
                let mut to_remove_vec: Vec<usize> = Vec::new();

                for val in write.iter_mut(){
                    let val = val.lock().await;
                    if val.creation_time.elapsed().unwrap() > val.expire_time {
                        warn!("CLEAN: {} B : {}", val.username, val.creation_time.elapsed().unwrap() <= val.expire_time);
                        to_remove_vec.push(i);
                        i += 1;
                    }
                }

                to_remove_vec.drain(0..).for_each(|i| {
                    write.remove(i);
                });
            }
        });

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
                let cache_vec: &State<Arc<RwLock<Vec<Mutex<AuthState>>>>> = try_outcome!(req.guard::<&State<Arc<RwLock<Vec<Mutex<AuthState>>>>>>().await);
                let user_id = match user_id.value().parse::<usize>() {
                    Ok(val) => val,
                    Err(_) => return Outcome::Failure((Status::InternalServerError, ())),
                };

                let cache_vec = cache_vec.read().await;

                return match cache_vec.get(user_id) {
                    Some(val) => {
                        let mut val = val.lock().await;
                        val.expire_time.add_assign(Duration::from_secs(3 * 60));

                        return Outcome::Success(val.clone())
                    },
                    None => Outcome::Failure((Status::Unauthorized, ())),
                };
            },
            None => {
                Outcome::Failure((Status::Unauthorized, ()))
            }
        }
    }
}