#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use serde_json::json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn new_task(task_name: String, priority: u8) {


    let task = Task {
        task_name,
        is_completed: false,
        priority,
        is_repeatable: false,
    };

    let mut file = match OpenOptions::new().read(true).open("data.json") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        },
    };

    let mut buffer = String::new();
    if let Err(e) = file.read_to_string(&mut buffer) {
        eprintln!("Error reading file: {}", e);
        return;
    }

    // Deserializa o conteúdo do arquivo JSON
    let mut json_value = serde_json::from_str(&buffer).unwrap_or_else(|_| json!({"tasks": {}}));

    // Adiciona a nova tarefa
    if let Some(tasks) = json_value.get_mut("tasks") {
        if let Some(tasks_obj) = tasks.as_object_mut() {
            tasks_obj.insert(task.task_name.clone(), json!(task));
        }
    }

    // Serializa o JSON atualizado de volta para uma string
    let updated_json = match serde_json::to_string_pretty(&json_value) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error serializing JSON: {}", e);
            return;
        },
    };

    // Abre o arquivo JSON para escrita, truncando o conteúdo existente
    let mut file = match OpenOptions::new().write(true).truncate(true).open("data.json") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file for writing: {}", e);
            return;
        },
    };
    // Escreve o JSON atualizado de volta ao arquivo
    if let Err(e) = file.write_all(updated_json.as_bytes()) {
        eprintln!("Error writing to file: {}", e);
    };
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![new_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    task_name: String,
    is_completed: bool,
    priority: u8,
    is_repeatable: bool, // not implemented yet
}