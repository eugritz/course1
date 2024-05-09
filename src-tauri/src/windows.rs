use tauri::Manager;

pub fn build_windows(app: &tauri::App) {
    tauri::WindowBuilder::new(
        app,
        "StatisticsWindow",
        tauri::WindowUrl::App(
            "src/views/StatisticsWindow/index.html".parse().unwrap(),
        ),
    )
    .title("course1 - Статистика")
    .visible(false)
    .build()
    .unwrap();

    tauri::WindowBuilder::new(
        app,
        "NewDeckDialog",
        tauri::WindowUrl::App(
            "src/views/NewDeckDialog/index.html".parse().unwrap(),
        ),
    )
    .title("Создать новую колоду")
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .inner_size(400.0, 140.0)
    .build()
    .unwrap();

    tauri::WindowBuilder::new(
        app,
        "RenameDeckDialog",
        tauri::WindowUrl::App(
            "src/views/RenameDeckDialog/index.html".parse().unwrap(),
        ),
    )
    .title("Переименовать колоду")
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .inner_size(400.0, 140.0)
    .build()
    .unwrap();
}

#[tauri::command]
pub async fn open_stats_window(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("StatisticsWindow") {
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

#[tauri::command]
pub async fn open_rename_deck_dialog(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("RenameDeckDialog") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
