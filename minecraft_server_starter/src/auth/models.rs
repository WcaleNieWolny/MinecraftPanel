use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use diesel::mysql::MysqlConnection;

use super::schema::users;

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub user_type: i16,
}

impl User {
    pub fn read(connection: &mut MysqlConnection) -> Vec<User> {
        users::table.load::<User>(connection).unwrap()
    }
}