#![allow(unused)]

pub mod state;
pub mod commands;
pub mod utils;

use crate::state::app_state::AppState;
use crate::commands::server::create_server;
use crate::commands::fetch_version::get_mc_versions;

#[cfg_attr(mobile, tauri::mobile_entry_point)]

#[tauri::command] //  to make it invocable from JS
fn greet(name: &str, email: &str) -> String {
    println!("Inside RUST code");
    format!("Hello, {}! you are loggedIn with email {}", name, email)
}

pub fn run() {
  tauri::Builder::default()
  .manage(AppState::default())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet, create_server, get_mc_versions])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
