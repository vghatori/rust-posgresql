use serde::Serialize;
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Message {
    pub title : String,
    pub description : String
}