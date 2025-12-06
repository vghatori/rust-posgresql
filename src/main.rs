

mod database;
mod middleware;

use database::{
    models::message::Message,
};

use axum::{Router, routing::get, serve, Json};
use tokio::net::TcpListener;
use tower_http::cors::Any;
use be_server::{handel_delete, handel_update, handle_create_message, handle_get_all_messages, handle_setup_db};
use middleware::cors::build_cors_permission;


async fn handle_message() -> Json<Message>{
    Json(
        Message {
            title : "Hello World 1".into(),
            description : "This is a test 2".into()
        }
    )
}

#[tokio::main]
async fn main() {
    let cors = build_cors_permission(Any);
    let db_api = handle_setup_db().await.db;
    let messages = handle_get_all_messages(&db_api)
        .await
        .unwrap_or_else(|err| {
            eprint!("{:?}", err);
            vec![]
        });
    for (index, message) in messages.iter().enumerate() {
        println!("message {} : {:#?}", index, message);
    }
    
    let webapp = Router::new().route("/message", get(handle_message)).layer(cors);
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    serve(listener, webapp).await.unwrap();
}

