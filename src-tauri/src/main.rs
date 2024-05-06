// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use migration::{Migrator, MigratorTrait};
use tauri::Manager;

mod windows;
use windows::open_new_deck_dialog;
use windows::open_stats;

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_stats,
            open_new_deck_dialog,
            api::decks::get_all_decks,
            api::decks::create_deck,
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
    let db = tauri::async_runtime::block_on(async move {
        let conn_str = format!("sqlite://{}/db.sqlite?mode=rwc", app_data_dir);
        let db = sea_orm::Database::connect(conn_str).await.unwrap();
        Migrator::up(&db, None).await.unwrap();
        db
    });
    app.manage(sea_orm::DbConn::from(db));

    tauri::WindowBuilder::new(
        &app,
        "Statistics",
        tauri::WindowUrl::App(
            "src/views/Statistics/index.html".parse().unwrap(),
        ),
    )
    .title("course1 - Статистика")
    .visible(false)
    .build()
    .unwrap();

    tauri::WindowBuilder::new(
        &app,
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

    app.run(|_, _| {});
}
