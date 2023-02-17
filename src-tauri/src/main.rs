#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn launch() -> String {
  let tmp = "Launching Arknights".to_string();
  println!("{}", tmp);
  tmp
}

#[tauri::command]
fn options() -> String {
  let tmp = "Opening options".to_string();
  println!("{}", tmp);
  tmp
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![launch, options])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
