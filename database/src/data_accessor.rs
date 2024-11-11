use crate::errors::Error as DBError;
use crate::models::{QueryResult, RemoteJob};
use sqlx::sqlite::SqliteQueryResult;
use sqlx::Error;
use tokio::runtime::Runtime;

/*
 * TODO: Try and make ths work!
pub async fn exec_query_async<T: QueryResult>(
    connection_string: &str,
    query: &str,
    result: &mut dyn QueryResult
) -> Result<T, DBError> {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:test.db").await;
    match pool {
        Ok(connection_pool) => {
            let _: Vec<&QueryResult> =
            sqlx::query_as!(T, "SELECT * FROM RemoteJobs")
                .fetch_all(&pool)
                .await?;
            // let query: SqliteQueryResult = sqlx::query(query)
            //     .execute(&connection_pool)
            //     .await
            //     .unwrap();

            //  let parsed: T = query.try_into();

            // let result = sqlx::query_as_with(sql, arguments)
            //     .fetch_all(connection_pool)
            //     .await;

            Ok(())
        }
        Err(_) => Err(DBError::DatabaseError {
            message: String::from("Error connecting to database"),
            code: 1,
        }),
    }
}
*/

pub async fn get_remote_jobs() -> Result<Vec<RemoteJob>, Error> {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:test.db").await?;
    let remote_jobs: Vec<RemoteJob> = sqlx::query_as!(RemoteJob, "
            SELECT 
             PKRemoteJobs as RemoteJobID,
             Name,
             Url,
             FKCategory as Category
            FROM RemoteJobs

        ")
        .fetch_all(&pool)
        .await?;

    Ok(remote_jobs)
}

pub fn get_remote_jobs_sync() -> Result<Vec<RemoteJob>, Error> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // Execute the future, blocking the current thread until completion
    rt.block_on(async { get_remote_jobs().await })
}
