use std::env::join_paths;

use database::*;
use models::RemoteJob;
const DB_PATH: &str = "test.db";

fn main() {
    if let Ok(db_file) = file_manager::create_file(DB_PATH, false) {}

    if let Ok(jobs) = database::data_accessor::get_remote_jobs_sync() {
        jobs.iter().for_each(|job| println!("{:?}", job));
    }

}
