use sqlx::Error;
use tokio::runtime::Runtime;

use crate::models::RemoteJob;

pub async fn get_remote_jobs() -> Result<Vec<RemoteJob>, Error> {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite:test.db").await?;
    let remote_jobs: Vec<RemoteJob> = sqlx::query_as!(RemoteJob, "SELECT * FROM RemoteJobs")
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
    rt.block_on(async {get_remote_jobs().await })
}
