use log::debug;
use sea_orm::DbConn;

use service::{EntryKindField, EntryKindFieldService};

#[tauri::command]
pub async fn get_entry_kind_fields(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
) -> Result<Vec<EntryKindField>, ()> {
    debug!("get_entry_kind_fields START");
    let result = EntryKindFieldService::find_fields_by_entry_kind_id(
        &state,
        entry_kind_id,
    )
    .await
    .map_err(|_| ());
    debug!("get_entry_kind_fields FINISH");
    return result;
}
