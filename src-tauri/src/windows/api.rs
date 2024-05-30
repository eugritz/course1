use log::debug;
use tauri::Manager;

use crate::dto::{
    ConfirmationModalInputPayload, ConfirmationModalOutputPayload,
    InputModalInputPayload, InputModalOutputPayload,
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
    id: Option<String>,
    title: String,
    message: String,
    loading: Option<bool>,
) {
    debug!("open_confirmation_modal");
    if let Some(confirmation_modal) =
        window.app_handle().get_window("ConfirmationModal")
    {
        confirmation_modal.set_title(&title).unwrap();
        confirmation_modal
            .emit_to(
                confirmation_modal.label(),
                "ConfirmationModal:setData",
                ConfirmationModalInputPayload {
                    id,
                    title,
                    message,
                    loading,
                    parent: window.label().to_owned(),
                },
            )
            .unwrap();

        confirmation_modal.show_modal();
    }
}

#[tauri::command]
pub async fn confirmation_modal_on_result(
    window: tauri::Window,
    id: Option<String>,
    button: i32,
    parent: String,
) {
    debug!("confirmation_modal_on_result START");
    if window.label() != "ConfirmationModal" {
        return;
    }

    window
        .app_handle()
        .emit_to(
            &parent,
            "ConfirmationModal:onResult",
            ConfirmationModalOutputPayload { id, button },
        )
        .unwrap();
    debug!("confirmation_modal_on_result FINISH");
}

#[tauri::command]
pub async fn open_input_modal(
    window: tauri::Window,
    id: Option<String>,
    title: String,
    label: String,
    value: Option<String>,
    placeholder: Option<String>,
    button_text: Option<String>,
    loading: Option<bool>,
) {
    debug!("open_input_modal");
    if let Some(input_modal) = window.app_handle().get_window("InputModal") {
        input_modal.set_title(&title).unwrap();
        input_modal
            .emit(
                "InputModal:setData",
                InputModalInputPayload {
                    id,
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

        input_modal.show_modal();
    }
}

#[tauri::command]
pub async fn input_modal_on_result(
    window: tauri::Window,
    id: Option<String>,
    input: String,
    parent: String,
) {
    if window.label() != "InputModal" {
        return;
    }

    window
        .app_handle()
        .emit_to(
            &parent,
            "InputModal:onResult",
            InputModalOutputPayload { id, input },
        )
        .unwrap();
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
pub async fn open_entry_kind_list_window(handle: tauri::AppHandle) {
    debug!("open_entry_kind_list_window");
    if let Some(window) = handle.get_window("EntryKindListWindow") {
        window.show_modal();
    }
}

#[tauri::command]
pub async fn open_entry_kind_field_list_window(handle: tauri::AppHandle) {
    debug!("open_entry_kind_field_list_window");
    if let Some(window) = handle.get_window("EntryKindFieldListWindow") {
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
