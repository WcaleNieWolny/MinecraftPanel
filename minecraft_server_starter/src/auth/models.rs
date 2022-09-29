use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use diesel::mysql::MysqlConnection;

use super::schema::users;

#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub user_type: i16,
}

impl User {
    pub fn read_all(connection: &mut MysqlConnection) -> Vec<User> {
        users::table.load::<User>(connection).unwrap()
    }

    pub fn read_by_username(username: &str, connection: &mut MysqlConnection) -> QueryResult<User> {
        users::table.filter(users::username.eq(username)).first(connection)
    }

    pub fn create(self, connection: &mut MysqlConnection) -> anyhow::Result<()>{
        let res = diesel::insert_into(users::table)
            .values(&self)
            .execute(connection);

        if res.is_ok() {
            Ok(())
        }else {
            Err(anyhow::Error::from(res.unwrap_err()))
        }
    }
}