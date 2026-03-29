use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path,
    },
    response::Response,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use crate::common::result::ApiResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSMessage {
    pub msg_type: String,
    pub task_id: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMessage {
    pub task_id: String,
    pub status: String,
    pub progress: u8,
    pub message: String,
    pub result: Option<serde_json::Value>,
}

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let mut receiver = socket.split().1;

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(ws_msg) = serde_json::from_str::<WSMessage>(&text) {
                    tracing::info!("Received WS message: {:?}", ws_msg);
                }
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

pub async fn get_task_progress(Path(task_id): Path<String>) -> impl axum::response::IntoResponse {
    ApiResponse::success(ProgressMessage {
        task_id,
        status: "processing".to_string(),
        progress: 50,
        message: "正在处理中...".to_string(),
        result: None,
    })
}
