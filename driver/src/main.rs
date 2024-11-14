use data_accessor::{Database, SqliteDatabase};
use database::*;
use models::RemoteJob;
use repositories::remote_jobs::RemoteJobRepository;
use sqlx::FromRow;
use std::error::Error;

const DB_PATH: &str = "test.db";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Ok(_) = file_manager::create_file(DB_PATH, false) {}

    let db: SqliteDatabase = Database::new(DB_PATH).await?;
    let remote_jobs: Vec<RemoteJob>  = db
        .query(
            "
                SELECT 
                 PKRemoteJobs as RemoteJobID,
                 Name,
                 Url,
                 FKCategory as Category
                FROM RemoteJobs
        ",
        ).await?;
        // .await?
        // .into_iter()
        // .map(|row| RemoteJob::from_row(&row))
        // .collect::<Result<Vec<RemoteJob>, _>>()?;

    // Print the jobs
    remote_jobs.iter().for_each(|job| println!("{:?}", job));

    Ok(())
}
