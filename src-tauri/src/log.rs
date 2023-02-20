use serde::Serialize;
use std::sync::RwLock;
use tauri::{Manager, Window};
use tracing::error;

pub static MAIN_WINDOW: RwLock<Option<Window>> = RwLock::new(None);

#[derive(Serialize, Debug, Clone)]
struct LogMessage {
  pub msg_type: MessageType,
  pub msg: String,
}

#[derive(Serialize, Debug, Clone)]
pub enum MessageType {
  Info,
  Error,
  Warn,
}

pub fn info<S>(msg: S)
where
  S: ToString,
{
  let window_option = match MAIN_WINDOW.read() {
    Ok(l) => l,
    Err(e) => {
      error!("Error locking main_window: {}", e);
      return;
    }
  };
  let window = match &*window_option {
    Some(w) => w,
    None => {
      error!("No MAIN_WINDOW");
      return;
    }
  };

  if let Err(e) = window.emit_all(
    "log",
    LogMessage {
      msg: msg.to_string(),
      msg_type: MessageType::Info,
    },
  ) {
    error!("Error emitting log event: {}", e)
  }
}
