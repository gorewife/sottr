// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod cmd;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Add commands here
        .invoke_handler(tauri::generate_handler![
            cmd::greet,
            cmd::my_custom_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
