use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use diesel::prelude::*;
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use diesel::sqlite::SqliteConnection;

use super::schema::users;

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub user_type: i16,
}

impl User {
    pub fn read_all(connection: &mut SqliteConnection) -> Vec<User> {
        users::table.load::<User>(connection).unwrap()
    }

    pub fn read_by_username(username: &str, connection: &mut SqliteConnection) -> QueryResult<User> {
        users::table.filter(users::username.eq(username)).first(connection)
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