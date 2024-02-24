// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::json;
use tauri::{Manager, State};

mod task_manager;

#[tauri::command]
async fn create_task(
    app: tauri::AppHandle,
    state: State<'_, task_manager::Manager>,
) -> Result<(), ()> {
    println!("{:?}", state.tasks);

    let new_task = task_manager::tasks::Task::new();
    state.tasks.lock().unwrap().add_task(new_task);

    Ok(())
}

#[tauri::command]
async fn show_tasks(
    app: tauri::AppHandle,
    state: State<'_, task_manager::Manager>,
) -> Result<String, ()> {
    Ok(json!(state.tasks.lock().unwrap().0).to_string())
}

#[tauri::command]
async fn save_tasks(
    app: tauri::AppHandle,
    state: State<'_, task_manager::Manager>,
) -> Result<(), ()> {
    state.save();

    // Ok(json!(state.tasks.lock().unwrap().0).to_string())

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(task_manager::Manager::new())
        .invoke_handler(tauri::generate_handler![
            create_task,
            show_tasks,
            save_tasks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
