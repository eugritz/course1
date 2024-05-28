use log::debug;
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
pub async fn open_entry_kind_filter_modal(handle: tauri::AppHandle) {
    debug!("open_entry_kind_filter_modal");
    if let Some(window) = handle.get_window("EntryKindFilterModal") {
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
