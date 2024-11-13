// pub enum QueryResult {
//     RemoteJobResult(RemoteJob),
//     UserResult(User)
// }

pub trait QueryResult {}

#[derive(Debug, sqlx::FromRow)]
pub struct RemoteJob {
    pub RemoteJobID: i64,
    pub Name: String,
    pub Url: String,
    pub Category: i64,
}
impl QueryResult for RemoteJob {}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    id: i32,
    username: String,
    email: String,
}
impl QueryResult for User {}
