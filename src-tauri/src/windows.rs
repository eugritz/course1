use tauri::Manager;

#[tauri::command]
pub async fn open_stats(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("Statistics") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

#[tauri::command]
pub async fn open_new_deck_dialog(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("NewDeckDialog") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
