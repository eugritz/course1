use log::{debug, error};
use sea_orm::DbConn;

use service::{EntryFieldValue, EntryFieldValuesService};

#[tauri::command]
pub async fn get_entry_field_values(
    state: tauri::State<'_, DbConn>,
    entry_id: i32,
) -> Result<Vec<EntryFieldValue>, ()> {
    debug!("get_entry_field_values CALL");
    let result = EntryFieldValuesService::find_entry_field_values_extra(
        state.inner(), 
        entry_id
    )
    .await
    .map_err(|err| {
        error!("get_entry_field_values ERROR {}", err.to_string());
    })?;

    debug!("get_entry_field_values SUCCESS");
    Ok(result)
}
