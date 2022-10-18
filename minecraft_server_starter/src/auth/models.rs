use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use diesel::{prelude::*};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use diesel::sqlite::SqliteConnection;

use super::schema::{users, sessions};

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub user_type: i16,
}

#[derive(AsChangeset, Queryable, Insertable, Debug)]
#[diesel(table_name = sessions)]
pub struct UserSession {
    pub id: Option<i32>,
    pub expiration: chrono::naive::NaiveDateTime, //this is bullshit - just let me use std type
    pub user_id: i32,
}

impl UserSession {

    pub fn new(expiration: chrono::naive::NaiveDateTime, user_id: i32) -> Self{
        Self {
            id: None,
            expiration, user_id 
        }
    }

    pub fn read_by_id(id: i32, connection: &mut SqliteConnection) -> QueryResult<UserSession> {
        sessions::table.filter(sessions::id::nullable(sessions::id).eq(id)).first(connection)
    }

    pub fn read_all(connection: &mut SqliteConnection) -> Vec<UserSession> {
        sessions::table.load::<UserSession>(connection).unwrap()
    }

    pub fn put(&self, connection: &mut SqliteConnection) -> anyhow::Result<UserSession>{
        diesel::insert_into(sessions::table)
            .values(self)
            .execute(connection)?;

        Ok(sessions::table.order(sessions::id.desc()).first(connection)?)
    }

    pub fn delete(self, connection: &mut SqliteConnection) -> bool {
        let id = match self.id {
            Some(val) => val,
            None =>  return false
        };

        diesel::delete(sessions::table.filter(sessions::id::nullable(sessions::id).eq(id))).execute(connection).is_ok()
    }
}

impl User {
    pub fn read_all(connection: &mut SqliteConnection) -> Vec<User> {
        users::table.load::<User>(connection).unwrap()
    }

    pub fn read_by_username(username: &str, connection: &mut SqliteConnection) -> QueryResult<User> {
        users::table.filter(users::username.eq(username)).first(connection)
    }

    pub fn read_by_id(id: i32, connection: &mut SqliteConnection) -> QueryResult<User> {
        users::table.filter(users::id.nullable().eq(id)).first(connection)
    }

    pub fn create(self, argon2: &Argon2, connection: &mut SqliteConnection) -> anyhow::Result<()>{
        //We need to hash the password
        let pwd = &self.password;
        let salt = SaltString::generate(&mut OsRng);
        
        let pwd_hash = match argon2.hash_password(pwd.as_bytes(), &salt) {
            Ok(val) => val,
            Err(_) => return Err(anyhow::Error::msg("Couldn't hash password"))
        };

        let pwd_hash_str = pwd_hash.serialize().to_string();   

        let hashed_user = Self {
            id: self.id,
            username: self.username,
            password: pwd_hash_str,
            user_type: self.user_type
        };

        let res = diesel::insert_into(users::table)
            .values(&hashed_user)
            .execute(connection);

        if res.is_ok() {
            Ok(())
        }else {
            Err(anyhow::Error::from(res.unwrap_err()))
        }
    }
}