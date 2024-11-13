use sqlx::{
    Database as SqlxDatabase, Error as SqlxError, Executor, IntoArguments, Pool, Row, Transaction,
};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    RowParsingError(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            DatabaseError::QueryError(msg) => write!(f, "Query error: {}", msg),
            DatabaseError::RowParsingError(msg) => write!(f, "Row parsing error: {}", msg),
        }
    }
}

impl Error for DatabaseError {}

impl From<SqlxError> for DatabaseError {
    fn from(error: SqlxError) -> Self {
        DatabaseError::QueryError(error.to_string())
    }
}

pub struct Database<DB: SqlxDatabase> {
    pool: Pool<DB>,
}

impl<DB: SqlxDatabase> Database<DB>
where
    // Database type must implement SqlxDatabase
    DB: SqlxDatabase,
    // Pool reference can be used as an Executor.
    for<'c> &'c Pool<DB>: Executor<'c, Database = DB>,
    // Database connection can be used as a Executor.
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB>,
{
    /// Creates a new Database instance
    /// Takes a string slice that represents the connection string.
    pub async fn new(connection_string: &str) -> Result<Self, DatabaseError> {
        let pool = Pool::connect(connection_string)
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        Ok(Database { pool })
    }

    /// Executes a query that returns a result set
    pub async fn query<'a>(
        &self,
        query: &'a str,
    ) -> Result<Vec<<DB as SqlxDatabase>::Row>, DatabaseError>
    where
        <DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, DB>,
    {
        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        Ok(rows)
    }

    /// Executes a query that returns a single row
    /// Returns None if no rows are found
    pub async fn query_one<'a>(
        &self,
        query: &'a str,
    ) -> Result<Option<<DB as SqlxDatabase>::Row>, DatabaseError>
    where
        <DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, DB>,
    {
        Ok(sqlx::query(query).fetch_optional(&self.pool).await?)
    }

    /// Executes a query that doesn't return a result set
    /// Returns the number of rows affected
    pub async fn execute<'a>(&self, query: &'a str) -> Result<DB::QueryResult, DatabaseError>
    where
        <DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, DB>,
    {
        let result = sqlx::query(query).execute(&self.pool).await?;

        Ok(result)
    }

    /// Executes a transaction with multiple queries
    /// Rolls back if any query fails
    pub async fn transaction<F, R>(&self, f: F) -> Result<R, DatabaseError>
    where
        F: for<'t> FnOnce(
            &'t Transaction<'t, DB>,
        ) -> futures::future::BoxFuture<'t, Result<R, DatabaseError>>,
    {
        let transaction = self.pool.begin().await?;

        match f(&transaction).await {
            Ok(result) => {
                transaction.commit().await?;
                Ok(result)
            }
            Err(e) => {
                transaction.rollback().await?;
                Err(e)
            }
        }
    }

    /// Returns a reference to the underlying connection pool
    pub fn pool(&self) -> &Pool<DB> {
        &self.pool
    }
}

// // Helper trait to convert row values to various types
// pub trait RowExt {
//     fn get_str(&self, column: &str) -> Result<String, DatabaseError>;
//     fn get_i32(&self, column: &str) -> Result<i32, DatabaseError>;
//     fn get_i64(&self, column: &str) -> Result<i64, DatabaseError>;
//     fn get_f64(&self, column: &str) -> Result<f64, DatabaseError>;
//     fn get_bool(&self, column: &str) -> Result<bool, DatabaseError>;
// }
// 
// impl<R: Row> RowExt for R {
//     fn get_str(&self, column: &str) -> Result<String, DatabaseError> {
//         self.try_get(column)
//             .map_err(|e| DatabaseError::RowParsingError(e.to_string()))
//     }
// 
//     fn get_i32(&self, column: &str) -> Result<i32, DatabaseError> {
//         self.try_get(column)
//             .map_err(|e| DatabaseError::RowParsingError(e.to_string()))
//     }
// 
//     fn get_i64(&self, column: &str) -> Result<i64, DatabaseError> {
//         self.try_get(column)
//             .map_err(|e| DatabaseError::RowParsingError(e.to_string()))
//     }
// 
//     fn get_f64(&self, column: &str) -> Result<f64, DatabaseError> {
//         self.try_get(column)
//             .map_err(|e| DatabaseError::RowParsingError(e.to_string()))
//     }
// 
//     fn get_bool(&self, column: &str) -> Result<bool, DatabaseError> {
//         self.try_get(column)
//             .map_err(|e| DatabaseError::RowParsingError(e.to_string()))
//     }
// }

// Type aliases for common database types
// pub type PostgresDatabase = Database<sqlx::Postgres>;
// pub type MySqlDatabase = Database<sqlx::MySql>;
pub type SqliteDatabase = Database<sqlx::Sqlite>;
