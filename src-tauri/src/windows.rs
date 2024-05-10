use gtk::prelude::*;
use tauri::Manager;

use crate::dto::{
    ConfirmationModalInputPayload, ConfirmationModalOutputPayload,
};

pub fn build_windows(app: &tauri::App) {
    let main_window = app.get_window("App").unwrap().gtk_window().unwrap();

    let window = tauri::WindowBuilder::new(
        app,
        "ConfirmationModal",
        tauri::WindowUrl::App(
            "src/views/ConfirmationModal/index.html".parse().unwrap(),
        ),
    )
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
    window.set_modal(true);

    tauri::WindowBuilder::new(
        app,
        "CardsWindow",
        tauri::WindowUrl::App(
            "src/views/CardsWindow/index.html".parse().unwrap(),
        ),
    )
    .title("course1 - Карты")
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
pub async fn open_confirmation_modal(
    handle: tauri::AppHandle,
    title: String,
    message: String,
    loading: bool,
) {
    if let Some(window) = handle.get_window("ConfirmationModal") {
        window.set_title(&title).unwrap();
        window
            .emit(
                "ConfirmationModal:setData",
                ConfirmationModalInputPayload {
                    title,
                    message,
                    loading,
                },
            )
            .unwrap();

        if let Some(parent) = handle.get_focused_window() {
            let parent = parent.gtk_window().unwrap();
            window
                .gtk_window()
                .unwrap()
                .set_transient_for(Some(&parent));
        }

        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

#[tauri::command]
pub async fn confirmation_modal_on_result(
    handle: tauri::AppHandle,
    button: i32,
) {
    if let Some(confirmation_window) = handle.get_focused_window() {
        if confirmation_window.label() != "ConfirmationModal" {
            return;
        }

        let parent = confirmation_window
            .gtk_window()
            .unwrap()
            .transient_for()
            .unwrap();

        for (_, window) in handle.windows() {
            let gtk_window = window.gtk_window().unwrap();
            if gtk_window == parent {
                window
                    .emit(
                        "ConfirmationModal:onResult",
                        ConfirmationModalOutputPayload { button },
                    )
                    .unwrap();
                return;
            }
        }
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

#[tauri::command]
pub async fn open_cards_window(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("CardsWindow") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
