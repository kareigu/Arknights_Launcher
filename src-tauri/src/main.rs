#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{sync::Arc, time::SystemTime};

use discord_sdk::{activity::Activity, Discord};
use runas::Command;
use std::error::Error;
use tauri::async_runtime::Mutex;

static APP_ID: i64 = 1062430347557613610;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn launch(client: tauri::State<'_, Arc<Mutex<Client>>>) -> Result<(), String> {
  println!("Launching Arknights");
  if !cfg!(target_os = "windows") {
    unimplemented!()
  }
  let mut command = Command::new("L:/Games/Arknights/Arknights.exe.lnk");
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
async fn stop(client: tauri::State<'_, Arc<Mutex<Client>>>) -> Result<(), String> {
  client
    .lock()
    .await
    .clear_activity()
    .await
    .map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
fn options() -> String {
  let tmp = "Opening options".to_string();
  println!("{}", tmp);
  tmp
}

#[tauri::command]
async fn has_activity(client: tauri::State<'_, Arc<Mutex<Client>>>) -> Result<bool, ()> {
  Ok(client.lock().await.activity_set)
}

#[tauri::command]
fn log(invoke_message: String) {
  println!("{}", invoke_message);
}

struct Client {
  pub discord: discord_sdk::Discord,
  pub user: discord_sdk::user::User,
  pub wheel: discord_sdk::wheel::Wheel,
  pub activity_set: bool,
}

impl Client {
  pub async fn clear_activity(&mut self) -> Result<Option<Activity>, discord_sdk::Error> {
    let activity = self.discord.clear_activity().await?;
    self.activity_set = false;
    Ok(activity)
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let (wheel, handler) = discord_sdk::wheel::Wheel::new(Box::new(|e| {
    println!("{}", e);
  }));

  let mut user = wheel.user();

  let discord = Discord::new(
    discord_sdk::DiscordApp::PlainId(APP_ID),
    discord_sdk::Subscriptions::ACTIVITY,
    Box::new(handler),
  )?;

  user.0.changed().await.unwrap();

  let user = match &*user.0.borrow() {
    discord_sdk::wheel::UserState::Connected(user) => user.clone(),
    discord_sdk::wheel::UserState::Disconnected(e) => {
      panic!("Failed to connect to discord: {}", e)
    }
  };

  let discord_client = Arc::new(Mutex::new(Client {
    discord,
    user,
    wheel,
    activity_set: false,
  }));

  tauri::Builder::default()
    .manage(discord_client.clone())
    .invoke_handler(tauri::generate_handler![
      launch,
      stop,
      options,
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
