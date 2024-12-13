use crate::{errors::Error as DBError, models::RemoteJob};

pub trait RemoteJobDataAccessor {
    async fn GetRemoteJobs(&self) -> Result<Vec<RemoteJob>, DBError>;
    async fn GetReomteJobByName(&self) -> Result<Vec<RemoteJob>, DBError>;
    async fn InsertRemoteJob(&self, remote_job: RemoteJob);
}
