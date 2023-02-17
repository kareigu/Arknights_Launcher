#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{sync::Arc, time::SystemTime};

use discord_sdk::Discord;
use runas::Command;

static APP_ID: i64 = 1062430347557613610;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn launch(client: tauri::State<'_, Arc<Client>>) -> Result<(), ()> {
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

  match client.discord.update_activity(activity).await {
    Ok(o) => println!("Activity updated {:?}", o),
    Err(e) => println!("Error updating activity: {}", e),
  };
  Ok(())
}

#[tauri::command]
fn options() -> String {
  let tmp = "Opening options".to_string();
  println!("{}", tmp);
  tmp
}

struct Client {
  pub discord: discord_sdk::Discord,
  pub user: discord_sdk::user::User,
  pub wheel: discord_sdk::wheel::Wheel,
}

#[tokio::main]
async fn main() {
  let (wheel, handler) = discord_sdk::wheel::Wheel::new(Box::new(|e| {
    println!("{}", e);
  }));

  let mut user = wheel.user();

  let discord = Discord::new(
    discord_sdk::DiscordApp::PlainId(APP_ID),
    discord_sdk::Subscriptions::ACTIVITY,
    Box::new(handler),
  )
  .expect("Couldn't create Discord client");

  user.0.changed().await.unwrap();

  let user = match &*user.0.borrow() {
    discord_sdk::wheel::UserState::Connected(user) => user.clone(),
    discord_sdk::wheel::UserState::Disconnected(e) => {
      panic!("Failed to connect to discord: {}", e)
    }
  };

  let discord_client = Arc::new(Client {
    discord,
    user,
    wheel,
  });

  tauri::Builder::default()
    .manage(discord_client.clone())
    .invoke_handler(tauri::generate_handler![launch, options])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  discord_client
    .discord
    .clear_activity()
    .await
    .expect("Failed clearing activity");
}
