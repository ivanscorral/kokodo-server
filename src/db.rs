use core::fmt;
use std::env;

use actix_web::HttpResponse;
use diesel::SqliteConnection;
use serde::Serialize;

const DEFAULT_DB_URL: &str = "sqlite::memory:";
const DB_URL_KEY: &str = "DATABASE_URL";


#[derive(Debug)]
pub enum DbError {
    ConnectionError(String),
    OperationError(diesel::result::Error),
}

#[derive(Debug)]
pub enum DbOperationResult<T> {
    Success(T),
    NotFound,
    InternalError(DbError),     
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::ConnectionError(e) => write!(f, "Database Connection Error: {}", e),
            DbError::OperationError(e) => write!(f, "Database Operation Error: {}", e),
        }
    }
}

pub fn get_database_url() -> Result<String, DbError> {
    env::var(DB_URL_KEY)
        .map_err(|_| DbError::ConnectionError(format!("Failed to fetch {}", DB_URL_KEY)))
        .or_else(|_| Ok(DEFAULT_DB_URL.to_string()))
}



pub fn execute_with_new_connection<T, F>(operation: F) -> DbOperationResult<T>
where
    F: FnOnce(&SqliteConnection) -> Result<T, diesel::result::Error>,
    T: Serialize,
{
    let db_url = match get_database_url() {
        Ok(url) => url,
        Err(e) => return DbOperationResult::InternalError(e),
    };
    // TODO: Use `db_url` to create a new connection and execute the operation
}
