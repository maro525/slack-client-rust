#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod service;

#[tauri::command]
async fn search_messages(query: String) -> Result<command::search_messages::Response, String> {
    // vec!["RRR".into(), "UUU".into(), "SSS".into(), "TTT".into()]
    command::search_messages::exec(query)
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_messages])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
