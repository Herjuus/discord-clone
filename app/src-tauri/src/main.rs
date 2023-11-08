// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
use crate::commands::{close_splashscreen, handle_sign_in};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![close_splashscreen, handle_sign_in])
    .plugin(tauri_plugin_store::Builder::default().build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
