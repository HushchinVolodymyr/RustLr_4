use std::sync::Arc;
use rocket::{routes};
use rocket_cors::{CorsOptions, AllowedOrigins};
use tokio::sync::broadcast;
use futures_util::{SinkExt, StreamExt};
use rocket::serde::json::serde_json;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use sqlx::PgPool;

mod services;
mod models;
mod handlers;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tokio::spawn(async {
        if let Err(e) = start_websocket_server().await {
            log::error!("WebSocket server failed: {}", e);
        }
    });

    let (sender, _) = broadcast::channel::<String>(100);

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_headers: rocket_cors::AllowedHeaders::all(),
        ..Default::default()
    }
        .to_cors()
        .expect("Error creating CORS middleware");

    rocket::build()
        .manage(WebSocketState {
            sender: sender.clone(),
        })
        .mount("/", routes![handlers::register_user, handlers::login_user, handlers::get_messages])
        .attach(cors) 
        .launch()
        .await?;

    Ok(())
}

#[derive(Clone)]
struct WebSocketState {
    sender: broadcast::Sender<String>, 
}

async fn start_websocket_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("WebSocket server is running on ws://127.0.0.1:8080");

    let (tx, _) = broadcast::channel::<String>(100);
    let pool = PgPool::connect(&services::get_database_url()).await?;

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let rx = tx.subscribe();
        let pool = pool.clone();

        tokio::spawn(handle_connection(stream, tx, rx, pool));
    }

    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
    pool: PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // Task to forward messages from the broadcast channel to the WebSocket
    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            if let Err(e) = ws_sender.send(Message::Text(Utf8Bytes::from(message))).await {
                eprintln!("Error sending message: {:?}", e);
            }
        }
    });

    // Wait for the first message to get the user ID
    let user_id = if let Some(Ok(Message::Text(text))) = ws_receiver.next().await {
        text.parse::<i32>()?
    } else {
        return Err("Failed to get user ID from initial message".into());
    };

    while let Some(message) = ws_receiver.next().await {
        let message = message?;

        if let Message::Text(text) = message {



            println!("Received message: {}", text.to_string().clone());

            // Broadcast the message to all subscribers
            if let Err(e) = tx.send(text.clone().parse().unwrap()) {
                eprintln!("Error broadcasting message: {:?}", e);
            }

            // Save the message to the database
            let rmessage = models::RMessage {
                chat_id: 1, // Replace with actual chat_id
                user_id: user_id,
                message: text.clone().parse().unwrap(),
            };
            if let Err(e) = services::save_message_to_db(rmessage, &pool).await {
                eprintln!("Error saving message to database: {:?}", e);
            }
        }
    }

    Ok(())
}