use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info, warn};

use crate::error::{OkxError, OkxResult};
use crate::types::ws::events::{WsApiResponse, WsConnectionType, WsDataEvent, WsEvent, WsMessage};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Establish a WebSocket connection to the given URL.
pub async fn connect(url: &str) -> OkxResult<WsStream> {
    let url = url::Url::parse(url).map_err(|e| OkxError::Ws(format!("Invalid WS URL: {e}")))?;

    let (ws_stream, _response) = connect_async(url.as_str())
        .await
        .map_err(|e| OkxError::Ws(format!("WS connection failed: {e}")))?;

    Ok(ws_stream)
}

/// Send a JSON message over the WebSocket.
pub async fn send_json(
    ws: &mut WsStream,
    msg: &impl serde::Serialize,
) -> OkxResult<()> {
    let text = serde_json::to_string(msg)?;
    debug!("WS send: {}", text);
    ws.send(Message::Text(text.into()))
        .await
        .map_err(|e| OkxError::Ws(format!("WS send failed: {e}")))?;
    Ok(())
}

/// Send a raw text message (for "ping").
pub async fn send_text(ws: &mut WsStream, text: &str) -> OkxResult<()> {
    ws.send(Message::Text(text.to_string().into()))
        .await
        .map_err(|e| OkxError::Ws(format!("WS send failed: {e}")))?;
    Ok(())
}

/// Parse an incoming WebSocket text message into a WsMessage.
pub fn parse_ws_message(text: &str) -> Option<WsMessage> {
    if text == "pong" {
        return Some(WsMessage::Pong);
    }

    let value: serde_json::Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to parse WS message as JSON: {e}");
            return None;
        }
    };

    // WS API responses include both `id` and `op`.
    if value.get("id").is_some() && value.get("op").is_some() {
        if let Ok(resp) = serde_json::from_value::<WsApiResponse>(value) {
            return Some(WsMessage::ApiResponse(resp));
        } else {
            return None;
        }
    }

    // Data events include `arg` and `data`.
    if value.get("arg").is_some() && value.get("data").is_some() {
        if let Ok(evt) = serde_json::from_value::<WsDataEvent>(value) {
            return Some(WsMessage::Data(evt));
        } else {
            return None;
        }
    }

    // Control events include `event`.
    if value.get("event").is_some() {
        if let Ok(evt) = serde_json::from_value::<WsEvent>(value) {
            return Some(WsMessage::Event(evt));
        } else {
            return None;
        }
    }

    warn!("Unknown WS message format: {text}");
    None
}

/// Splits a WebSocket stream and spawns write and read I/O tasks.
///
/// This is a synchronous function so callers can avoid holding
/// non-`Send` stream halves across `.await` points in their own
/// async state machines.
///
/// Returns `(write_tx, msg_rx)`: a channel for sending outbound
/// messages and a channel for receiving parsed inbound messages.
pub fn spawn_io_tasks(
    ws: WsStream,
    conn_type: WsConnectionType,
) -> (
    mpsc::UnboundedSender<String>,
    mpsc::UnboundedReceiver<WsMessage>,
) {
    let (mut write_half, read_half) = ws.split();
    let (write_tx, mut write_rx) = mpsc::unbounded_channel::<String>();
    let (msg_tx, msg_rx) = mpsc::unbounded_channel::<WsMessage>();
    let msg_tx_for_read = msg_tx.clone();

    tokio::spawn(async move {
        while let Some(msg) = write_rx.recv().await {
            if let Err(e) = write_half
                .send(Message::Text(msg.into()))
                .await
            {
                error!("WS {conn_type} write error: {e}");
                break;
            }
        }
        debug!("WS {conn_type} write loop ended");
    });

    tokio::spawn(async move {
        let mut read = read_half;
        while let Some(result) = read.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    if let Some(parsed) = parse_ws_message(&text) {
                        if msg_tx_for_read.send(parsed).is_err() {
                            break;
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    let _ = msg_tx_for_read.send(WsMessage::Disconnected(conn_type));
                    break;
                }
                Err(e) => {
                    error!("WS {conn_type} read error: {e}");
                    let _ = msg_tx_for_read.send(WsMessage::Disconnected(conn_type));
                    break;
                }
                _ => {}
            }
        }
    });

    (write_tx, msg_rx)
}

/// Run the message read loop for a WebSocket connection.
/// Reads messages from the WebSocket and sends them to the channel.
pub async fn read_loop(
    mut ws: WsStream,
    conn_type: WsConnectionType,
    tx: mpsc::UnboundedSender<WsMessage>,
) {
    info!("WS {conn_type} read loop started");

    while let Some(msg_result) = ws.next().await {
        match msg_result {
            Ok(Message::Text(text)) => {
                if let Some(parsed) = parse_ws_message(&text) {
                    if tx.send(parsed).is_err() {
                        debug!("WS {conn_type} receiver dropped, exiting read loop");
                        break;
                    }
                }
            }
            Ok(Message::Close(_)) => {
                info!("WS {conn_type} received close frame");
                let _ = tx.send(WsMessage::Disconnected(conn_type));
                break;
            }
            Ok(Message::Ping(data)) => {
                debug!("WS {conn_type} received ping");
                // Tungstenite auto-responds with `pong`.
                let _ = data;
            }
            Ok(_) => {}
            Err(e) => {
                error!("WS {conn_type} read error: {e}");
                let _ = tx.send(WsMessage::Disconnected(conn_type));
                break;
            }
        }
    }

    info!("WS {conn_type} read loop ended");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pong() {
        let msg = parse_ws_message("pong");
        assert!(matches!(msg, Some(WsMessage::Pong)));
    }

    #[test]
    fn test_parse_data_event() {
        let json = r#"{"arg":{"channel":"tickers","instId":"BTC-USDT"},"data":[{"instId":"BTC-USDT","last":"50000"}]}"#;
        let msg = parse_ws_message(json);
        assert!(matches!(msg, Some(WsMessage::Data(_))));
        if let Some(WsMessage::Data(evt)) = msg {
            assert_eq!(evt.arg.channel, "tickers");
            assert_eq!(evt.data.len(), 1);
        }
    }

    #[test]
    fn test_parse_event() {
        let json = r#"{"event":"subscribe","arg":{"channel":"tickers","instId":"BTC-USDT"}}"#;
        let msg = parse_ws_message(json);
        assert!(matches!(msg, Some(WsMessage::Event(_))));
        if let Some(WsMessage::Event(evt)) = msg {
            assert_eq!(evt.event, "subscribe");
        }
    }

    #[test]
    fn test_parse_login_event() {
        let json = r#"{"event":"login","code":"0","msg":""}"#;
        let msg = parse_ws_message(json);
        assert!(matches!(msg, Some(WsMessage::Event(_))));
        if let Some(WsMessage::Event(evt)) = msg {
            assert_eq!(evt.event, "login");
            assert_eq!(evt.code.as_deref(), Some("0"));
        }
    }

    #[test]
    fn test_parse_api_response() {
        let json = r#"{"id":"1","op":"order","code":"0","msg":"","data":[{"ordId":"12345"}]}"#;
        let msg = parse_ws_message(json);
        assert!(matches!(msg, Some(WsMessage::ApiResponse(_))));
        if let Some(WsMessage::ApiResponse(resp)) = msg {
            assert_eq!(resp.id, "1");
            assert_eq!(resp.op, "order");
            assert_eq!(resp.code, "0");
        }
    }

    #[test]
    fn test_parse_invalid_json() {
        let msg = parse_ws_message("not json");
        assert!(msg.is_none());
    }

    #[test]
    fn test_parse_unknown_format() {
        let json = r#"{"foo":"bar"}"#;
        let msg = parse_ws_message(json);
        assert!(msg.is_none());
    }
}
