use tauri::Manager;
use gtk::prelude::*;

pub fn build_windows(app: &tauri::App) {
    let main_window = app.get_window("App").unwrap().gtk_window().unwrap();

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

    let window = tauri::WindowBuilder::new(
        app,
        "NewDeckModal",
        tauri::WindowUrl::App(
            "src/views/NewDeckModal/index.html".parse().unwrap(),
        ),
    )
    .title("Создать новую колоду")
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .inner_size(400.0, 140.0)
    .build()
    .unwrap()
    .gtk_window()
    .unwrap();
    window.set_icon(None);
    window.set_transient_for(Some(&main_window));
    window.set_modal(true);

    let window = tauri::WindowBuilder::new(
        app,
        "RenameDeckModal",
        tauri::WindowUrl::App(
            "src/views/RenameDeckModal/index.html".parse().unwrap(),
        ),
    )
    .title("Переименовать колоду")
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .inner_size(400.0, 140.0)
    .build()
    .unwrap()
    .gtk_window()
    .unwrap();
    window.set_icon(None);
    window.set_transient_for(Some(&main_window));
    window.set_modal(true);
}

#[tauri::command]
pub async fn open_stats_window(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("StatisticsWindow") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

#[tauri::command]
pub async fn open_new_deck_modal(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("NewDeckModal") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

#[tauri::command]
pub async fn open_rename_deck_modal(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("RenameDeckModal") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
