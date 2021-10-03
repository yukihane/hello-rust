#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PersonalData {
  name: String,
  birthDay: DateTime<Utc>,
}

#[tauri::command]
fn greet(personal_data: PersonalData) {
  println!("I was invoked from JS! {:?}", personal_data);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
