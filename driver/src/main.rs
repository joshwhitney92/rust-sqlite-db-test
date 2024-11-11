use std::env::join_paths;

use database::*;
use models::RemoteJob;
use repositories::remote_jobs::{RemoteJobRepository, TRemoteJobRepository};
const DB_PATH: &str = "test.db";

fn main() {
    if let Ok(db_file) = file_manager::create_file(DB_PATH, false) {}

    let repo = RemoteJobRepository::new();

    // if let Ok(jobs) = database::data_accessor::get_remote_jobs_sync() {
    match repo.GetRemoteJobs() {
        Ok(jobs) => {
            jobs.iter().for_each(|job| println!("{:?}", job));
        },
        Err(x) => {
            println!("{:?}", x.to_string())
        }
    }
    // if let Ok(jobs) = repo.GetRemoteJobs() {
    //     jobs.iter().for_each(|job| println!("{:?}", job));
    // }
}
