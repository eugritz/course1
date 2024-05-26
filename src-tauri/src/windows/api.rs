use tauri::Manager;

use crate::dto::{
    ConfirmationModalInputPayload, ConfirmationModalOutputPayload,
};
use crate::windows::ext::*;

#[tauri::command]
pub async fn window_close(window: tauri::Window) {
    if window.url().path() == "/src/index.html" {
        window.app_handle().exit(0);
        return;
    }

    window.set_modal(false);
    window.hide().unwrap();
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

        window.show_modal();
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

        if let Some(parent) = get_parent_window(&confirmation_window) {
            parent
                .emit(
                    "ConfirmationModal:onResult",
                    ConfirmationModalOutputPayload { button },
                )
                .unwrap();
        }
    }
}

#[tauri::command]
pub async fn open_new_deck_modal(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("NewDeckModal") {
        window.show_modal();
    }
}

#[tauri::command]
pub async fn open_rename_deck_modal(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("RenameDeckModal") {
        window.show_modal();
    }
}

#[tauri::command]
pub async fn open_cards_window(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("CardsWindow") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

#[tauri::command]
pub async fn open_add_window(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("AddWindow") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

#[tauri::command]
pub async fn open_deck_filter_modal(handle: tauri::AppHandle) {
    if let Some(window) = handle.get_window("DeckFilterModal") {
        window.show_modal();
    }
}
