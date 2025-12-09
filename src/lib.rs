pub mod database;
pub mod middleware;

pub use database::repository::web::{
    WebRepository,
    PgWebRepository
};
use std::env;
use dotenv::dotenv;
use sqlx::Error;
use sqlx::postgres::PgPoolOptions;
use crate::database::models::message::Message;

#[derive(Clone)]
pub struct AppState {
    pub db : WebRepository
}

pub async fn method_setup_db() -> AppState{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DB_URL must be set");
    AppState {
        db : WebRepository {
            pool : PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
                .expect("Failed to connect to database")
        }
    }
}

pub async fn method_create_message(action : &WebRepository, message : Message) {
    action.create(
        message
    ).await.expect("create message failed");
}

pub async fn method_get_all_messages(action: &WebRepository) -> Result<Vec<Message>, Error> {
    action.read_all().await
}

pub async fn method_update(action : &WebRepository, message_id : i32, title : String, description : String) {
    action.update(message_id, title, description).await.expect("update message failed");
}

pub async fn method_delete(action : &WebRepository, message_id : i32) {
    action.delete(message_id).await.expect("delete message failed");
}