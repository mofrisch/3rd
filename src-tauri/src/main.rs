// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use tauri_plugin_log::LogTarget;

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::Webview];

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];

#[tauri::command]
fn interpret(input: String) -> String {
    info!("Received input: {}", input);
    input.to_string()
}

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(LOG_TARGETS)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![interpret])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
