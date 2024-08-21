// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        SaltString
    },
    Argon2
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_db(db_name: String, master_password: String) -> String {
    let password = master_password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let mut output_key_material = [0u8; 256];
    Argon2::default().hash_password_into(password, salt.to_string().as_bytes(), &mut output_key_material).unwrap();

    match fs::write(db_name, output_key_material) {
        Ok(_) => format!("Database file created successfully"),
        Err(e) => format!("Database creation failed: {}", e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
