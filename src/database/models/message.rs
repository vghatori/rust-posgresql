use serde::{Serialize, Deserialize};
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Message {
    pub title : String,
    pub description : String
}