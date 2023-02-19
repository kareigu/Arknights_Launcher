#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{sync::Arc, time::SystemTime};

use runas::Command;
use std::error::Error;
use tauri::async_runtime::Mutex;

mod options;
use options::Options;

mod discord;

static APP_ID: i64 = 1062430347557613610;
static OPTIONS_PATH: &str = "options.ron";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn launch(
  client: tauri::State<'_, Arc<Mutex<discord::Client>>>,
  options: tauri::State<'_, Arc<Mutex<Options>>>,
) -> Result<(), String> {
  println!("Launching Arknights");
  if !cfg!(target_os = "windows") {
    unimplemented!()
  }

  let executable_path = options.lock().await.executable_path.clone();

  let mut command = Command::new(executable_path);
  let output = command.status();

  println!("{:?}", output);

  let activity = discord_sdk::activity::ActivityBuilder::default()
    .details("Playing")
    .state("Online")
    .assets(discord_sdk::activity::Assets::default().large("amiya", Some("Arknights".to_owned())))
    .start_timestamp(SystemTime::now());

  let mut client_lock = client.lock().await;
  let activity = client_lock
    .discord
    .update_activity(activity)
    .await
    .map_err(|e| e.to_string())?;
  println!("Updated activity: {:?}", activity);
  client_lock.activity_set = true;
  Ok(())
}

#[tauri::command]
async fn stop(client: tauri::State<'_, Arc<Mutex<discord::Client>>>) -> Result<(), String> {
  client
    .lock()
    .await
    .clear_activity()
    .await
    .map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
async fn options(options: tauri::State<'_, Arc<Mutex<Options>>>) -> Result<Options, ()> {
  Ok(options.lock().await.clone())
}

#[tauri::command]
async fn set_options(
  options: tauri::State<'_, Arc<Mutex<Options>>>,
  new_options: Options,
) -> Result<(), ()> {
  let mut options_lock = options.lock().await;
  println!("new_options: {:?}", new_options);
  *options_lock = new_options;
  if let Err(e) = options_lock.save_to_file(OPTIONS_PATH) {
    println!("Error saving {OPTIONS_PATH}: {}", e);
  }

  Ok(())
}

#[tauri::command]
async fn has_activity(client: tauri::State<'_, Arc<Mutex<discord::Client>>>) -> Result<bool, ()> {
  Ok(client.lock().await.activity_set)
}

#[tauri::command]
fn log(msg: String) {
  println!("{}", msg);
}

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
      launch,
      stop,
      options,
      set_options,
      has_activity,
      log
    ])
    .run(tauri::generate_context!())?;

  let mut discord_client_lock = discord_client.lock().await;
  discord_client_lock
    .clear_activity()
    .await
    .expect("Failed clearing activity");

  Ok(())
}
