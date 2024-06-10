// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_task(name: String) -> String {
    println!("Task {} was created", name);
    format!("Task was created")
}

#[tauri::command]
fn calculate_sum(a: i32, b: i32) {
    println!("{}", a+b);
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_task, calculate_sum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
