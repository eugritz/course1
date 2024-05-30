use log::debug;
use sea_orm::DbConn;

use entity;
use service::EntryKindService;

#[tauri::command]
pub async fn get_all_entry_kinds(
    state: tauri::State<'_, DbConn>,
) -> Result<Vec<entity::entry_kinds::Model>, ()> {
    debug!("get_all_entry_kinds START");
    let result = EntryKindService::find_all_entry_kinds(&state)
        .await
        .or(Ok(vec![]));
    debug!("get_all_entry_kinds FINISH");
    return result;
}

#[tauri::command]
pub async fn create_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
    entry_kind_name: String,
) -> Result<entity::entry_kinds::Model, ()> {
    debug!("create_entry_kind START");
    let entry_kind =
        EntryKindService::find_entry_kind_by_id(&state, entry_kind_id).await;

    if entry_kind.is_err() || entry_kind.unwrap().is_none() {
        return Err(());
    }

    let result = EntryKindService::create_entry_kind(
        &state,
        entity::entry_kinds::Model {
            id: 0,
            name: entry_kind_name,
            default: false,
        },
    )
    .await
    .map(|x| x.try_into().unwrap())
    .map_err(|_| ());

    // TODO: clone fields
    // TODO: clone cards

    debug!("create_entry_kind FINISH");
    return result;
}

#[tauri::command]
pub async fn rename_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
    new_entry_kind_name: String,
) -> Result<entity::entry_kinds::Model, ()> {
    debug!("rename_entry_kind START");
    let result = EntryKindService::update_entry_kind_by_id(
        &state,
        entry_kind_id,
        entity::entry_kinds::Model {
            id: 0,
            name: new_entry_kind_name,
            default: false,
        },
    )
    .await
    .map_err(|_| ());
    debug!("rename_entry_kind FINISH");
    return result;
}

#[tauri::command]
pub async fn delete_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
) -> Result<(), ()> {
    debug!("delete_entry_kind START");
    EntryKindService::delete_entry_kind(&state, entry_kind_id)
        .await
        .map_err(|_| ())?;
    debug!("delete_entry_kind FINISH");
    Ok(())
}

#[tauri::command]
pub async fn last_entry_kind(
    state: tauri::State<'_, DbConn>,
) -> Result<Option<entity::entry_kinds::Model>, ()> {
    debug!("last_entry_kind START");
    let result = EntryKindService::find_last_entry_kind(&state)
        .await
        .map(|x| x)
        .map_err(|_| ());
    debug!("last_entry_kind FINISH");
    return result;
}
