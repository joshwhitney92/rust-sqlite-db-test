use data_accessor::{Database, SqliteDatabase};
use database::*;
use interface::remote_job_data_accessor::RemoteJobDataAccessor;
use models::RemoteJob;
use repositories::remote_jobs::RemoteJobRepository;
use std::error::Error;

const DB_PATH: &str = "test.db";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Ok(_) = file_manager::create_file(DB_PATH, false) {}

    // Use the Repo
    println!("Using the Repo pattern");
    let repo = RemoteJobRepository::new();
    match repo.GetRemoteJobs().await {
        Ok(jobs) => {
            for job in jobs.iter() {
                println!("{:?}", job)
            }
        }
        Err(e) => {
            println!("Error: {:?}", e.to_string());
        }
    }

    println!("Using the Database abstraction");
    // TODO: Move this query to it's own file.
    let db_query = r#"
                 SELECT 
                  PKRemoteJobs as RemoteJobID,
                  Name,
                  Url,
                  FKCategory as Category
                 FROM RemoteJobs
     "#;

    let db: SqliteDatabase = Database::new(DB_PATH).await?;
    let remote_jobs: Vec<RemoteJob> = db.query(db_query).await?;

    // Print the jobs
    remote_jobs.iter().for_each(|job| println!("{:?}", job));

    // Print a single job
    println!("\rHere is a single job:");
    let single_job: Option<RemoteJob> = db.query_one(db_query).await?;
    single_job.iter().for_each(|job| println!("{:?}", job));

    Ok(())
}
