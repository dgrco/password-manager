// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::write;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_db(db_name: String, master_password: String) -> String {
    match write(db_name, master_password) {
        Ok(_) => format!("Database created"),
        Err(e) => format!("Database creation failed: {}", e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
