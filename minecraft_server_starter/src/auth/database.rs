//SRC: https://github.com/sean3z/rocket-diesel-rest-api-example/blob/master/src/db.rs
use std::ops::{Deref, DerefMut};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::outcome::{Outcome};

use crate::config::ServerConfig;

// An alias to the type for a pool of Diesel Mysql Connection
pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

/// Initialize the database pool.
pub fn connect(config: ServerConfig) -> MysqlPool {
    let manager = ConnectionManager::<MysqlConnection>::new(config.mysql_string);
    Pool::new(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub PooledConnection<ConnectionManager<MysqlConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Connection {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) ->   request::Outcome<Connection, Self::Error> {
        let pool = req.guard::<&State<Pool<ConnectionManager<MysqlConnection>>>>().await;
        match pool {
            Outcome::Success(conn) => Outcome::Success(Connection(conn.inner().try_get().unwrap())),
            Outcome::Failure(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
            Outcome::Forward(_) =>  Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &Connection as an &MysqlConnection.
impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Connection {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
