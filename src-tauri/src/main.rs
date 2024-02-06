// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use reginald::{item::RegiItem, Reginald};
use setup::desetup;
use tauri::{async_runtime::Mutex, RunEvent};

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

    tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("Error building app")
        .run(move |_, event| match event {
            RunEvent::ExitRequested { .. } => {
                if let Err(e) = desetup() {
                    eprintln!("Failed destup: {}", e);
                } else {
                    println!("Finished desetup");
                }
            }      
            _ => {}
        });

}
