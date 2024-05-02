// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod windows;
use windows::open_stats;

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_stats])
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

    tauri::WindowBuilder::new(
        &app,
        "Statistics",
        tauri::WindowUrl::App("src/views/Statistics/index.html".parse().unwrap()),
    )
    .title("course1 - Статистика")
    .visible(false)
    .build()
    .unwrap();

    app.run(|_, _| {});
}
