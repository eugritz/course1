use tauri::Manager;

#[allow(dead_code)]
#[tauri::command]
pub async fn open_stats(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("Statistics") {
        window.show().unwrap();
    }
}
