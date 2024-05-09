// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use migration::{Migrator, MigratorTrait};
use tauri::Manager;

mod windows;
use windows::build_windows;

mod dto;

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            windows::open_confirmation_modal,
            windows::confirmation_modal_on_result,
            windows::open_new_deck_modal,
            windows::open_rename_deck_modal,
            windows::open_stats_window,
            api::decks::get_all_decks,
            api::decks::create_deck,
            api::decks::rename_deck,
            api::decks::delete_deck,
        ])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if event.window().url().path() == "/src/index.html" {
                    event.window().app_handle().exit(0);
                    return;
                }

                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let app_data_dir = app
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    fs::create_dir_all(app_data_dir.clone())
        .expect("failed to create dir in app_data_dir");

    let db = tauri::async_runtime::block_on(async move {
        let conn_str = format!("sqlite://{}/db.sqlite?mode=rwc", app_data_dir);
        let db = sea_orm::Database::connect(conn_str)
            .await
            .expect("error while connecting to database");
        Migrator::up(&db, None).await.unwrap();
        db
    });
    app.manage(sea_orm::DbConn::from(db));

    build_windows(&app);

    app.run(|_, _| {});
}
