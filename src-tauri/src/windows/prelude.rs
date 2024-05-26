use tauri::Manager;

use crate::windows::ext::*;

pub fn build_windows(app: &tauri::App) {
    let main_window = app.get_window("App").unwrap();

    tauri::WindowBuilder::new(
        app,
        "ConfirmationModal",
        tauri::WindowUrl::App(
            "src/views/ConfirmationModal/index.html".parse().unwrap(),
        ),
    )
    .ancestor(&main_window)
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .inner_size(400.0, 140.0)
    .build()
    .unwrap();

    tauri::WindowBuilder::new(
        app,
        "NewDeckModal",
        tauri::WindowUrl::App(
            "src/views/NewDeckModal/index.html".parse().unwrap(),
        ),
    )
    .ancestor(&main_window)
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
        "RenameDeckModal",
        tauri::WindowUrl::App(
            "src/views/RenameDeckModal/index.html".parse().unwrap(),
        ),
    )
    .ancestor(&main_window)
    .title("Переименовать колоду")
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .inner_size(400.0, 140.0)
    .build()
    .unwrap();

    tauri::WindowBuilder::new(
        app,
        "CardsWindow",
        tauri::WindowUrl::App(
            "src/views/CardsWindow/index.html".parse().unwrap(),
        ),
    )
    .ancestor(&main_window)
    .title("course1 - Просмотр")
    .visible(false)
    .inner_size(1300.0, 600.0)
    .min_inner_size(1000.0, 500.0)
    .build()
    .unwrap();

    let add_window = tauri::WindowBuilder::new(
        app,
        "AddWindow",
        tauri::WindowUrl::App(
            "src/views/AddWindow/index.html".parse().unwrap(),
        ),
    )
    .ancestor(&main_window)
    .title("course1 - Добавить")
    .visible(false)
    .inner_size(800.0, 600.0)
    .build()
    .unwrap();

    tauri::WindowBuilder::new(
        app,
        "DeckFilterModal",
        tauri::WindowUrl::App(
            "src/views/DeckFilterModal/index.html".parse().unwrap(),
        ),
    )
    .ancestor(&add_window)
    .title("Выбор колоды")
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .build()
    .unwrap();
}

pub fn rebuild_windows(handle: &tauri::AppHandle) {
    if let Some(confirmation_window) = handle.get_window("ConfirmationModal") {
        confirmation_window.close().unwrap();
    }

    tauri::WindowBuilder::new(
        handle,
        "ConfirmationModal",
        tauri::WindowUrl::App(
            "src/views/ConfirmationModal/index.html".parse().unwrap(),
        ),
    )
    .ancestor(&handle.get_focused_window().unwrap())
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .inner_size(400.0, 140.0)
    .build()
    .unwrap();
}
