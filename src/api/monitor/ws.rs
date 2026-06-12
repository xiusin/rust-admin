use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use tokio::time::{interval, Duration};

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut ticker = interval(Duration::from_secs(10));
    
    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let msg = json!({
                    "title": "System Notification",
                    "content": "This is a real-time message from the WebSocket server.",
                    "time": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
                });
                
                if socket.send(Message::Text(msg.to_string())).await.is_err() {
                    break;
                }
            }
            msg = socket.recv() => {
                if let Some(msg) = msg {
                    if let Ok(Message::Close(_)) = msg {
                        break;
                    }
                    if let Ok(Message::Text(text)) = msg {
                        // Echo back received text as a notification for testing
                        let response = json!({
                            "title": "Received Message",
                            "content": format!("Server received: {}", text),
                            "time": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
                        });
                        let _ = socket.send(Message::Text(response.to_string())).await;
                    }
                } else {
                    break;
                }
            }
        }
    }
}
