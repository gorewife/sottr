#[tauri::command]
pub fn my_custom_command() {
    println!("Greetings from JavaScript!");
}