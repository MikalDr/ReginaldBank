// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use reginald::{item::RegiItem, Reginald};
use tauri::async_runtime::Mutex;

mod reginald;

mod setup;

static REGINALD: Lazy<Mutex<Reginald>> = Lazy::new(|| Mutex::new(Reginald::new()));

#[tauri::command]
async fn get_bof() -> Vec<RegiItem> {
    REGINALD.lock().await.items().clone()
}

#[tokio::main]
async fn main() {
    if let Err(err) = setup::setup().await {
        eprintln!("Failed setup: {}", err);
    } else {
        println!("Setup complete");
    }
    // WTF?
    // let _ = setup::desetup().await;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_bof])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    if let Err(err) = setup::desetup().await {
        eprintln!("Failed desetup: {}", err);
    } else {
        println!("Desetup complete");
    }
}
