use log::{debug, error};
use sea_orm::DbConn;

use entity;
use service::EntryKindService;

#[tauri::command]
pub async fn get_all_entry_kinds(
    state: tauri::State<'_, DbConn>,
) -> Result<Vec<entity::entry_kinds::Model>, ()> {
    debug!("get_all_entry_kinds CALL");
    let result = EntryKindService::find_all_entry_kinds(state.inner()).await;

    match result {
        Ok(result) => {
            debug!("get_all_entry_kinds SUCCESS");
            Ok(result)
        }
        Err(error) => {
            error!("get_all_entry_kinds ERROR {}", error.to_string());
            Ok(vec![])
        }
    }
}

#[tauri::command]
pub async fn create_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
    entry_kind_name: String,
) -> Result<entity::entry_kinds::Model, ()> {
    debug!("create_entry_kind CALL");
    let entry_kind =
        EntryKindService::find_entry_kind_by_id(state.inner(), entry_kind_id)
            .await;

    if entry_kind.is_err() || entry_kind.unwrap().is_none() {
        return Err(());
    }

    let result = EntryKindService::create_entry_kind(
        state.inner(),
        entity::entry_kinds::Model {
            id: 0,
            name: entry_kind_name,
            immutable: false,
        },
    )
    .await
    .map(|x| x.try_into().unwrap());

    // TODO: clone fields
    // TODO: clone cards

    match result {
        Ok(result) => {
            debug!("create_entry_kind SUCCESS");
            Ok(result)
        }
        Err(error) => {
            error!("create_entry_kind ERROR {}", error.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn rename_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
    new_entry_kind_name: String,
) -> Result<entity::entry_kinds::Model, ()> {
    debug!("rename_entry_kind CALL");
    let result = EntryKindService::update_entry_kind_by_id(
        state.inner(),
        entry_kind_id,
        entity::entry_kinds::Model {
            id: 0,
            name: new_entry_kind_name,
            immutable: false,
        },
    )
    .await;

    match result {
        Ok(result) => {
            debug!("rename_entry_kind SUCCESS");
            Ok(result)
        }
        Err(error) => {
            error!("rename_entry_kind ERROR {}", error.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn delete_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
) -> Result<(), ()> {
    debug!("delete_entry_kind CALL");
    let result =
        EntryKindService::delete_entry_kind(state.inner(), entry_kind_id).await;

    match result {
        Ok(_) => {
            debug!("delete_entry_kind SUCCESS");
            Ok(())
        }
        Err(error) => {
            error!("delete_entry_kind ERROR {}", error.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn last_entry_kind(
    state: tauri::State<'_, DbConn>,
) -> Result<Option<entity::entry_kinds::Model>, ()> {
    debug!("last_entry_kind CALL");
    let result = EntryKindService::find_last_entry_kind(state.inner()).await;

    match result {
        Ok(result) => {
            debug!("last_entry_kind SUCCESS");
            Ok(result)
        }
        Err(error) => {
            error!("last_entry_kind ERROR {}", error.to_string());
            Err(())
        }
    }
}
