


use axum::{Json, Router, routing::{get, post}, serve};
use tokio::net::TcpListener;
use tower_http::cors::Any;

use be_server::{
    method_create_message, method_get_all_messages, method_setup_db,
    database::models::message::Message,
    middleware::cors::build_cors_permission
};

async fn handle_message() -> Json<Vec<Message>>{
    let db_api = method_setup_db().await.db;
    let messages = method_get_all_messages(&db_api)
        .await
        .unwrap_or_else(|err| {
            eprint!("{:?}", err);
            vec![]
        });
    Json(messages)
}

async fn handle_create_message(Json(payload): Json<Message>) {
    print!("Payload received: {:?}", payload);
    let db_api = method_setup_db().await.db;
    method_create_message(&db_api, payload).await;
}

#[tokio::main]
async fn main() {
    let cors = build_cors_permission(Any);
    let db_api = method_setup_db().await.db;

    let messages = method_get_all_messages(&db_api)
        .await
        .unwrap_or_else(|err| {
            eprint!("{:?}", err);
            vec![]
        });
    for (index, message) in messages.iter().enumerate() {
        println!("message {} : {:#?}", index, message);
    }
    
    let webapp = Router::new()
    .route("/message", get(handle_message))
    .route("/create", post(handle_create_message))
    .layer(cors);
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    serve(listener, webapp).await.unwrap();
}

