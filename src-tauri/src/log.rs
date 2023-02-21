use serde::Serialize;
use std::sync::{Mutex, RwLock};
use tauri::{Manager, Window};
use tracing::{error, warn};

pub static MAIN_WINDOW: RwLock<Option<Window>> = RwLock::new(None);

static LOG_STORAGE: Mutex<Vec<LogMessage>> = Mutex::new(Vec::new());

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
  log(MessageType::Info, msg.to_string());
}

pub fn warn<S>(msg: S)
where
  S: ToString,
{
  log(MessageType::Warn, msg.to_string());
}

pub fn error<S>(msg: S)
where
  S: ToString,
{
  log(MessageType::Error, msg.to_string());
}

fn log(msg_type: MessageType, msg: String) {
  let window_option = match MAIN_WINDOW.read() {
    Ok(l) => l,
    Err(e) => {
      error!("Error locking MAIN_WINDOW: {}", e);
      return;
    }
  };

  let message = LogMessage { msg, msg_type };

  let send_message = |w: &Window, payload: LogMessage| {
    if let Err(e) = w.emit_all("log", payload) {
      error!("Error emitting log event: {}", e)
    }
  };

  let window = match &*window_option {
    Some(w) => {
      let mut log_storage_lock = match LOG_STORAGE.lock() {
        Ok(l) => l,
        Err(e) => {
          error!("Error locking LOG_STORAGE: {}", e);
          return;
        }
      };

      if log_storage_lock.is_empty() {
        w
      } else {
        for msg in log_storage_lock.iter() {
          send_message(w, msg.clone());
        }

        log_storage_lock.clear();

        w
      }
    }
    None => {
      warn!("No MAIN_WINDOW");

      let mut log_storage_lock = match LOG_STORAGE.lock() {
        Ok(l) => l,
        Err(e) => {
          error!("Error locking LOG_STORAGE: {}", e);
          return;
        }
      };

      log_storage_lock.push(message);
      return;
    }
  };

  send_message(window, message);
}
