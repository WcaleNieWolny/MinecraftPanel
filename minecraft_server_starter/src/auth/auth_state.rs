use std::{time::{SystemTime, Duration}, sync::Arc, ops::AddAssign, collections::HashMap};
use rand::distributions::{Alphanumeric, DistString};
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
    time: Arc<Mutex<AuthStateTime>>,
    pub username: String,
    pub user_type: UserType,
    pub web_socket_auth_token: String
}

#[derive(Clone, Debug)]
struct AuthStateTime{
    creation_time: SystemTime,
    expire_time: Duration,
}

impl AuthState{

    pub fn new(db_user: User) -> anyhow::Result<Self>{

        let hash = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

        Ok(
            Self {
                time: Arc::new(Mutex::new(
                    AuthStateTime{
                        creation_time: SystemTime::now(), 
                        expire_time: Duration::from_secs(60 * 60), 
                    }
                )),
                username: db_user.username, 
                user_type: match db_user.user_type {
                    0 => UserType::NORMAL,
                    1 => UserType::ADMIN,
                    _ => return Err(anyhow::Error::msg("Invalid user type in the database"))
                },
                web_socket_auth_token: hash
            }
        )
    }

    pub async fn put_in_cache(self, cache: &State<Arc<RwLock<HashMap<String, AuthState>>>>) -> String{
        let mut lock = cache.write().await;
        let id = format!("{}_{}", self.username.clone(), Alphanumeric.sample_string(&mut rand::thread_rng(), 16));
        lock.insert(id.clone(), self);
        return id;
    }
}

pub fn stage(auth_vec: Arc<RwLock<HashMap<String, AuthState>>>) -> AdHoc {

    AdHoc::on_ignite("Auth StateStage", |rocket| async {

        let auth_vec_clone = auth_vec.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10 * 60));

            loop {
                interval.tick().await;

                let mut write = auth_vec_clone.write().await;
                let mut to_remove_vec: Vec<String> = Vec::new();

                for (index, val) in write.iter_mut(){
                    let time = val.time.lock().await;
                    if time.creation_time.elapsed().unwrap() > time.expire_time {
                        to_remove_vec.push(index.clone());
                    }
                }

                to_remove_vec.drain(0..).for_each(|i| {
                    write.remove(&i);
                });
            }
        });

        rocket.manage(auth_vec)
    })
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthState {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<AuthState, Self::Error> {
        let user_id = req.cookies().get_private("user_id");

        match user_id {
            Some(user_id) => {
                let cache_vec: &State<Arc<RwLock<HashMap<String, AuthState>>>> = try_outcome!(req.guard::<&State<Arc<RwLock<HashMap<String, AuthState>>>>>().await);
                let user_id = user_id.value().to_owned();

                let cache_vec = cache_vec.read().await;

                return match cache_vec.get(&user_id) {
                    Some(val) => {
                        val.time.lock().await.expire_time.add_assign(Duration::from_secs(3 * 60));

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
