//SRC: https://github.com/sean3z/rocket-diesel-rest-api-example/blob/master/src/db.rs
use std::ops::{Deref, DerefMut};
use argon2::Argon2;
use rand::distributions::{Alphanumeric, DistString};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::outcome::{Outcome};

use crate::config::ServerConfig;

// An alias to the type for a pool of Diesel Mysql Connection
pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

/// Initialize the database pool.
pub fn connect(config: ServerConfig, argon2: &Argon2) -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(config.mysql_string);
    let pool = Pool::new(manager).expect("Failed to create pool");

    let mut connection = Connection(pool.try_get().unwrap());
    check_admin(argon2, &mut connection);

    pool
}

fn check_admin(argon2: &Argon2, connection: &mut Connection){
    let admin_user = super::models::User::read_by_username("admin", connection);
    
    if admin_user.is_err() {
        let password = Alphanumeric.sample_string(&mut rand::thread_rng(), 15);
        println!("---------------------------");
        println!(" ADMIN USER DOES NOT EXIST ");
        println!(" PANEL WILL CREATE IT NOW! ");
        println!(" PASSWORD: {}", password);
        println!("");
        println!(" PLEASE NOTE THAT DOWN!!!! ");
        println!("---------------------------");

        let user = super::models::User {
            id: None,
            username: "admin".to_string(),
            password,
            user_type: 1,
        };

        user.create(argon2, connection).expect("Couldn't create admin user!");
    }
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub PooledConnection<ConnectionManager<SqliteConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Connection {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) ->   request::Outcome<Connection, Self::Error> {
        let pool = req.guard::<&State<Pool<ConnectionManager<SqliteConnection>>>>().await;
        match pool {
            Outcome::Success(conn) => Outcome::Success(Connection(conn.inner().try_get().unwrap())),
            Outcome::Failure(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
            Outcome::Forward(_) =>  Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &Connection as an &MysqlConnection.
impl Deref for Connection {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Connection {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
