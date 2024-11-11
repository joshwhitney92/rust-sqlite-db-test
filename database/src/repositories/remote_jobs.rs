use crate::errors::Error as DBError;
use crate::models::RemoteJob;

pub trait TRemoteJobRepository {
    fn GetRemoteJobs(&self) -> Result<Vec<RemoteJob>, DBError>;
    fn GetReomteJobByName(&self) -> Result<Vec<RemoteJob>, DBError>;
    fn InsertRemoteJob(&self, remote_job: RemoteJob);
}

pub struct RemoteJobRepository {}

impl TRemoteJobRepository for RemoteJobRepository {
    fn GetRemoteJobs(&self) -> Result<Vec<RemoteJob>, DBError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        // Execute the future, blocking the current thread until completion
        let results = rt.block_on(async {
            if let Ok(pool) = sqlx::sqlite::SqlitePool::connect("sqlite:test.db").await {
                if let Ok(remote_jobs) = sqlx::query_as!(
                    RemoteJob,
                    "
                    SELECT 
                     PKRemoteJobs as RemoteJobID,
                     Name,
                     Url,
                     FKCategory as Category
                    FROM RemoteJobs
                "
                )
                .fetch_all(&pool)
                .await
                {
                    Ok(remote_jobs)
                } else {
                    Err(DBError::DatabaseError {
                        message: String::from("somehting went wrong"),
                        code: 1,
                    })
                }
            } else {
                Err(DBError::DatabaseError {
                    message: String::from("somehting went wrong"),
                    code: 1,
                })
            }
        });

        match results {
            Ok(jobs) => Ok(jobs),
            _ => Err(DBError::DatabaseError {
                message: "Problem loading Jobs!".to_string(),
                code: 1,
            }),
        }
    }

    fn GetReomteJobByName(&self) -> Result<Vec<RemoteJob>, DBError> {
        todo!()
    }

    fn InsertRemoteJob(&self, remote_job: RemoteJob) {
        todo!()
    }
}

impl RemoteJobRepository {
    pub fn new() -> Self {
        RemoteJobRepository {}
    }
}
