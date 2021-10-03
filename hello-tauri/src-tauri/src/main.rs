#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use chrono::{DateTime, Datelike, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct PersonalData {
  name: String,
  birthDay: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
struct Response {
  message: String,
}

#[tauri::command]
fn greet(personal_data: PersonalData) -> Response {
  println!("recieve: {:?}", personal_data);
  let age: i32 = calc_age(&personal_data.birthDay, &Local::now());
  let message = format!("こんにちは, {}({}歳)！", personal_data.name, age);

  Response { message }
}

fn calc_age(birth_day: &DateTime<Utc>, now: &DateTime<Local>) -> i32 {
  let year = now.year() - birth_day.year();
  let delta = match now.month() as i32 - birth_day.month() as i32 {
    m if m > 0 => 0,
    m if m < 0 => -1,
    _ => {
      if now.day() as i32 - birth_day.day() as i32 >= 0 {
        0
      } else {
        -1
      }
    }
  };

  year + delta
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
