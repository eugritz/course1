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
        "InputModal",
        tauri::WindowUrl::App(
            "src/views/InputModal/index.html".parse().unwrap(),
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
        "EntryWindow",
        tauri::WindowUrl::App(
            "src/views/EntryWindow/index.html".parse().unwrap(),
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
    .min_inner_size(555.0, 500.0)
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
    .inner_size(400.0, 300.0)
    .build()
    .unwrap();

    let entry_kind_filter_modal = tauri::WindowBuilder::new(
        app,
        "EntryKindFilterModal",
        tauri::WindowUrl::App(
            "src/views/EntryKindFilterModal/index.html".parse().unwrap(),
        ),
    )
    .ancestor(&add_window)
    .title("Выбор вида записи")
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .inner_size(400.0, 300.0)
    .build()
    .unwrap();

    let entry_kind_list_modal = tauri::WindowBuilder::new(
        app,
        "EntryKindListWindow",
        tauri::WindowUrl::App(
            "src/views/EntryKindListWindow/index.html".parse().unwrap(),
        ),
    )
    .ancestor(&entry_kind_filter_modal)
    .title("Виды записей")
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .inner_size(500.0, 300.0)
    .build()
    .unwrap();

    tauri::WindowBuilder::new(
        app,
        "EntryKindAddModal",
        tauri::WindowUrl::App(
            "src/views/EntryKindAddModal/index.html".parse().unwrap(),
        ),
    )
    .ancestor(&entry_kind_list_modal)
    .title("Добавить вид записи")
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .inner_size(350.0, 250.0)
    .build()
    .unwrap();

    tauri::WindowBuilder::new(
        app,
        "EntryKindFieldListWindow",
        tauri::WindowUrl::App(
            "src/views/EntryKindFieldListWindow/index.html"
                .parse()
                .unwrap(),
        ),
    )
    .title("Поля вида записи")
    .visible(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .inner_size(600.0, 400.0)
    .build()
    .unwrap();
}
