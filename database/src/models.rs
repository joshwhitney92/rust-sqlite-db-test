


#[derive(Debug, sqlx::FromRow)]
pub struct RemoteJob {
    pub PKRemoteJobs: i64,
    pub Name: String,
    pub Url: String,
    pub FKCategory: i64,
}


#[derive(Debug, sqlx::FromRow)]
pub struct User {
    id: i32,
    username: String,
    email: String,
}
