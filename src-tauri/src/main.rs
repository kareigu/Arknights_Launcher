#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::error::Error;
use std::sync::Arc;
use tauri::async_runtime::Mutex;

mod commands;
mod discord;
mod options;
use options::Options;

pub static APP_ID: i64 = 1062430347557613610;
pub static OPTIONS_PATH: &str = "options.ron";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let discord_client = discord::Client::new(APP_ID).await?;
  let discord_client = Arc::new(Mutex::new(discord_client));

  let config = match Options::load_from_file(OPTIONS_PATH) {
    Ok(o) => {
      println!("Loaded options.ron successfully");
      o
    }
    Err(e) => {
      println!("Error loading {OPTIONS_PATH}: {}", e);
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
      commands::log
    ])
    .run(tauri::generate_context!())?;

  let mut discord_client_lock = discord_client.lock().await;
  discord_client_lock
    .clear_activity()
    .await
    .expect("Failed clearing activity");

  Ok(())
}
