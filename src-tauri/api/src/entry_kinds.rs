use sea_orm::DbConn;

use entity;
use service::EntryKindService;

#[tauri::command]
pub async fn get_all_entry_kinds(
    state: tauri::State<'_, DbConn>,
) -> Result<Vec<entity::entry_kinds::Model>, ()> {
    EntryKindService::find_all_entry_kinds(&state).await.or(Ok(vec![]))
}

#[tauri::command]
pub async fn create_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_name: String,
) -> Result<entity::entry_kinds::Model, ()> {
    EntryKindService::create_entry_kind(
        &state,
        entity::entry_kinds::Model {
            id: 0,
            name: entry_kind_name,
        },
    )
    .await
    .map(|x| x.try_into().unwrap())
    .map_err(|_| ())
}

#[tauri::command]
pub async fn rename_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
    new_entry_kind_name: String,
) -> Result<entity::entry_kinds::Model, ()> {
    EntryKindService::update_entry_kind_by_id(
        &state,
        entry_kind_id,
        entity::entry_kinds::Model {
            id: 0,
            name: new_entry_kind_name,
        },
    )
    .await
    .map_err(|_| ())
}

#[tauri::command]
pub async fn delete_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
) -> Result<(), ()> {
    EntryKindService::delete_entry_kind(&state, entry_kind_id)
        .await
        .map_err(|_| ())?;
    Ok(())
}

#[tauri::command]
pub async fn last_entry_kind(
    state: tauri::State<'_, DbConn>,
) -> Result<Option<entity::entry_kinds::Model>, ()> {
    EntryKindService::find_last_entry_kind(&state)
    .await
    .map(|x| x)
    .map_err(|_| ())
}
