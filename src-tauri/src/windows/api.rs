use log::debug;
use tauri::Manager;

use crate::dto::{
    ConfirmationModalInputPayload,
    ConfirmationModalOutputPayload,
    InputModalInputPayload,
    InputModalOutputPayload,
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
    window: tauri::Window,
    title: String,
    message: String,
    loading: bool,
) {
    debug!("open_confirmation_modal");
    if let Some(window) = window.app_handle().get_window("ConfirmationModal") {
        window.set_title(&title).unwrap();
        window
            .emit(
                "ConfirmationModal:setData",
                ConfirmationModalInputPayload {
                    title,
                    message,
                    loading,
                    parent: window.label().to_owned(),
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
    parent: String,
) {
    if let Some(confirmation_window) = handle.get_focused_window() {
        if confirmation_window.label() != "ConfirmationModal" {
            return;
        }

        if let Some(parent) = handle.get_window(parent.as_str()) {
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
pub async fn open_input_modal(
    window: tauri::Window,
    title: String,
    label: String,
    value: Option<String>,
    placeholder: String,
    button_text: String,
    loading: bool,
) {
    debug!("open_input_modal");
    if let Some(window) = window.app_handle().get_window("InputModal") {
        window.set_title(&title).unwrap();
        window
            .emit(
                "InputModal:setData",
                InputModalInputPayload {
                    title,
                    label,
                    value,
                    placeholder,
                    buttonText: button_text,
                    loading,
                    parent: window.label().to_owned(),
                },
            )
            .unwrap();

        window.show_modal();
    }
}

#[tauri::command]
pub async fn input_modal_on_result(
    handle: tauri::AppHandle,
    input: String,
    parent: String,
) {
    if let Some(input_window) = handle.get_focused_window() {
        if input_window.label() != "InputModal" {
            return;
        }

        if let Some(parent) = handle.get_window(parent.as_str()) {
            parent
                .emit(
                    "InputModal:onResult",
                    InputModalOutputPayload { input },
                )
                .unwrap();
        }
    }
}

#[tauri::command]
pub async fn open_deck_new_modal(handle: tauri::AppHandle) {
    debug!("open_deck_new_modal");
    if let Some(window) = handle.get_window("DeckNewModal") {
        window.show_modal();
    }
}

#[tauri::command]
pub async fn open_deck_rename_modal(handle: tauri::AppHandle) {
    debug!("open_deck_rename_modal");
    if let Some(window) = handle.get_window("DeckRenameModal") {
        window.show_modal();
    }
}

#[tauri::command]
pub async fn open_deck_filter_modal(handle: tauri::AppHandle) {
    debug!("open_deck_filter_modal");
    if let Some(window) = handle.get_window("DeckFilterModal") {
        window.show_modal();
    }
}

#[tauri::command]
pub async fn open_entry_kind_add_modal(handle: tauri::AppHandle) {
    debug!("open_entry_kind_add_modal");
    if let Some(window) = handle.get_window("EntryKindAddModal") {
        window.show_modal();
    }
}

#[tauri::command]
pub async fn open_entry_kind_filter_modal(handle: tauri::AppHandle) {
    debug!("open_entry_kind_filter_modal");
    if let Some(window) = handle.get_window("EntryKindFilterModal") {
        window.show_modal();
    }
}

#[tauri::command]
pub async fn open_entry_kind_list_modal(handle: tauri::AppHandle) {
    debug!("open_entry_kind_list_modal");
    if let Some(window) = handle.get_window("EntryKindListModal") {
        window.show_modal();
    }
}

#[tauri::command]
pub async fn open_cards_window(handle: tauri::AppHandle) {
    debug!("open_cards_window");
    if let Some(window) = handle.get_window("CardsWindow") {
        window.show_ext();
    }
}

#[tauri::command]
pub async fn open_add_window(handle: tauri::AppHandle) {
    debug!("open_add_window");
    if let Some(window) = handle.get_window("AddWindow") {
        window.show_ext();
    }
}
