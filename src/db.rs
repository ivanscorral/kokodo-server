use core::fmt;
use std::env;

use tokio::task;    
use diesel::{SqliteConnection, Connection};
use serde::Serialize;

const DEFAULT_DB_URL: &str = "sqlite::memory:";
const DB_URL_KEY: &str = "DATABASE_URL";


#[derive(Debug)]
pub enum DbError {
    ConnectionError(String),
    OperationError(diesel::result::Error),
    InternalError(String),
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
            DbError::InternalError(e) => write!(f, "Internal Error: {}", e),
        }
    }
}

async fn get_database_url() -> Result<String, DbError> {
    env::var(DB_URL_KEY)
    .map_err(|_| DbError::ConnectionError(format!("Environment variable {} not found", DB_URL_KEY)))
    .or_else(|_| Ok(DEFAULT_DB_URL.to_string()))
}



pub async fn execute_with_new_connection<T, F>(operation: F) -> DbOperationResult<T>
where
    F: FnOnce(&SqliteConnection) -> Result<T, diesel::result::Error> + Send + 'static,
    T: Serialize + Send + 'static,
{
    let db_url = match get_database_url().await {
        Ok(url) => url,
        Err(e) => return DbOperationResult::InternalError(e),
    };
    // TODO: Use `db_url` to create a new connection and execute the operation
    let operation_result: Result<T, DbError> = task::spawn_blocking(move || {
        let connection = match SqliteConnection::establish(&db_url) {
            Ok(conn) => conn,
            Err(e) => return Err(DbError::ConnectionError(format!("Failed to establish connection: {}", e))),
        };
        match operation(&connection) {
            Ok(result) => Ok(result),
            Err(e) => Err(DbError::OperationError(e)),
        }
    }).await.unwrap_or_else(|_| Err(DbError::InternalError("Task join error".into())));
    
    match operation_result {
        Ok(result) => DbOperationResult::Success(result),
        Err(e) => DbOperationResult::InternalError(e),
    }
    
}
