use std::{time::{SystemTime}, sync::Arc, ops::{AddAssign, Add}, collections::HashMap};
use chrono::{NaiveDateTime, Local, NaiveTime, Duration};
use rand::distributions::{Alphanumeric, DistString};
use rocket::{http::{Status}, fairing::AdHoc};
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use rocket::outcome::{Outcome};
use rocket::outcome::try_outcome;
use tokio::sync::{RwLock, Mutex};

use super::{models::{User, UserSession}, database::Connection};

#[derive(Clone, Debug)]
pub enum UserType {
    NORMAL,
    ADMIN
}

#[derive(Clone, Debug)]
pub struct AuthState{
    pub username: String,
    pub user_type: UserType,
}

#[derive(Clone, Debug)]
struct AuthStateTime{
    creation_time: SystemTime,
    expire_time: Duration,
}

impl AuthState{

    pub fn new(db_user: &User) -> anyhow::Result<Self>{

        let hash = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

        Ok(
            Self {
                username: db_user.username.clone(), 
                user_type: match db_user.user_type {
                    0 => UserType::NORMAL,
                    1 => UserType::ADMIN,
                    _ => return Err(anyhow::Error::msg("Invalid user type in the database"))
                }
            }
        )
    }

    pub fn create_session(user: &User, connection: &mut Connection) -> anyhow::Result<String>{
        let naive_date_time = chrono::Utc::now().naive_utc() + Duration::hours(3);
        let session = UserSession::new(
            naive_date_time,
            user.id.unwrap()
        );

        let session = session.put(connection)?;

        Ok(session.id.unwrap().to_string())
    }
}

pub fn stage() -> AdHoc {

    AdHoc::on_ignite("Auth StateStage", |rocket| async {

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));

            loop {
                interval.tick().await;
                //TODO!
            }
        });

        rocket
    })
}

impl AuthState {
    pub async fn logout(&self){
        //TODO!
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthState {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<AuthState, Self::Error> {
        let user_id = req.cookies().get_private("user_id");

        match user_id {
            Some(user_id) => {
                let mut connection: Connection  = try_outcome!(req.guard().await);
                let user_id = match user_id.value().parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => return Outcome::Failure((Status::InternalServerError, ())),
                };

                

                let session = match UserSession::read_by_id(user_id, &mut connection) {
                    Ok(val) => val,
                    Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
                };

                let user = match User::read_by_id(session.user_id, &mut connection) {
                    Ok(val) => val,
                    Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
                };


                let state = match AuthState::new(&user) {
                    Ok(val) => val,
                    Err(_) => return Outcome::Failure((Status::InternalServerError, ())),
                };

                Outcome::Success(state)
            },
            None => {
                Outcome::Failure((Status::Unauthorized, ()))
            }
        }
    }
}
