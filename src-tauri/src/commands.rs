use crate::discord;
use crate::Options;
use runas::Command;
use std::{sync::Arc, time::SystemTime};
use tauri::async_runtime::Mutex;
use tracing::{error, info};

#[tauri::command]
pub async fn launch(
  client: tauri::State<'_, Arc<Mutex<discord::Client>>>,
  options: tauri::State<'_, Arc<Mutex<Options>>>,
) -> Result<(), String> {
  info!("Launching Arknights");
  if !cfg!(target_os = "windows") {
    unimplemented!()
  }

  let executable_path = options.lock().await.executable_path.clone();

  let mut command = Command::new(executable_path);
  let output = command.status();

  info!("Run output: {:?}", output);

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
  info!("Updated activity: {:?}", activity);
  crate::log::info("Discord activity set");
  client_lock.activity_set = true;
  Ok(())
}

#[tauri::command]
pub async fn stop(client: tauri::State<'_, Arc<Mutex<discord::Client>>>) -> Result<(), String> {
  client
    .lock()
    .await
    .clear_activity()
    .await
    .map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
pub async fn options(options: tauri::State<'_, Arc<Mutex<Options>>>) -> Result<Options, ()> {
  Ok(options.lock().await.clone())
}

#[tauri::command]
pub async fn set_options(
  options: tauri::State<'_, Arc<Mutex<Options>>>,
  new_options: Options,
) -> Result<(), ()> {
  let mut options_lock = options.lock().await;
  info!("new_options: {:?}", new_options);
  *options_lock = new_options;
  let options_path = Options::path();
  if let Err(e) = options_lock.save_to_file(options_path.clone()) {
    error!("Error saving {options_path}: {}", e);
  }

  crate::log::info("Options saved");
  Ok(())
}

#[tauri::command]
pub async fn has_activity(
  client: tauri::State<'_, Arc<Mutex<discord::Client>>>,
) -> Result<bool, ()> {
  Ok(client.lock().await.activity_set)
}

#[tauri::command]
pub async fn user(
  client: tauri::State<'_, Arc<Mutex<discord::Client>>>,
) -> Result<discord::UserData, ()> {
  Ok(client.lock().await.user_data())
}

#[tauri::command]
pub fn initialise() {
  crate::log::info("Initialised");
}
