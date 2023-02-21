#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::error::Error;
use std::sync::Arc;
use tauri::async_runtime::Mutex;

mod commands;
mod discord;
mod log;
mod options;
use options::Options;
use tracing::{error, info};

pub static APP_ID: i64 = 1062430347557613610;
pub static OPTIONS_PATH: &str = "options.ron";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  tracing_subscriber::fmt()
    .compact()
    .with_max_level(tracing::Level::TRACE)
    .init();

  let discord_client = discord::Client::new(APP_ID).await?;
  let discord_client = Arc::new(Mutex::new(discord_client));
  log::info("Discord connection initialised");

  let config = match Options::load_from_file(OPTIONS_PATH) {
    Ok(o) => {
      let msg = format!("Loaded {OPTIONS_PATH} successfully");
      info!(msg);
      log::info(msg);
      o
    }
    Err(e) => {
      error!("Error loading {OPTIONS_PATH}: {}", e);
      log::error(format!("Error loading {OPTIONS_PATH}"));
      log::warn("Using default settings");
      Options::default()
    }
  };
  let config_arc = Arc::new(Mutex::new(config));

  tauri::Builder::default()
    .manage(discord_client.clone())
    .manage(config_arc)
    .invoke_handler(tauri::generate_handler![
      commands::launch,
      commands::stop,
      commands::options,
      commands::set_options,
      commands::has_activity,
      commands::user,
      commands::initialise,
    ])
    .on_page_load(|window, _| {
      let mut main_window_lock = log::MAIN_WINDOW.write().unwrap();
      *main_window_lock = Some(window.clone());
    })
    .run(tauri::generate_context!())?;

  let mut discord_client_lock = discord_client.lock().await;
  discord_client_lock
    .clear_activity()
    .await
    .expect("Failed clearing activity");

  Ok(())
}
