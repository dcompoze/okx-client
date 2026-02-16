use std::time::Duration;

use tokio::sync::mpsc;
use tracing::debug;

/// Heartbeat ping sender. Sends "ping" at the configured interval.
/// Stops when the stop_rx receives a signal or the sender is dropped.
pub async fn heartbeat_loop(
    tx: mpsc::UnboundedSender<String>,
    interval: Duration,
    mut stop_rx: tokio::sync::oneshot::Receiver<()>,
) {
    let mut ticker = tokio::time::interval(interval);
    // Skip the first immediate tick.
    ticker.tick().await;

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                debug!("Sending WS ping");
                if tx.send("ping".to_string()).is_err() {
                    break;
                }
            }
            _ = &mut stop_rx => {
                debug!("Heartbeat stopped");
                break;
            }
        }
    }
}
