#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod service;

#[tauri::command]
async fn search_messages(
    query: String,
) -> Result<command::search_messages::CommandResponse, String> {
    command::search_messages::exec(query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn hello() -> String {
    command::hello::exec()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello, search_messages])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
