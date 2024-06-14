use entity::entry_field_values;
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
        entry_id,
    )
    .await
    .map_err(|err| {
        error!("get_entry_field_values ERROR {}", err.to_string());
    })?;

    debug!("get_entry_field_values SUCCESS");
    Ok(result)
}

#[tauri::command]
pub async fn update_entry_field_value(
    state: tauri::State<'_, DbConn>,
    entry_field_value_id: i32,
    new_value: String,
) -> Result<(), ()> {
    debug!("update_entry_field_value CALL");
    EntryFieldValuesService::update_entry_field_value(
        state.inner(),
        entry_field_values::Model {
            id: entry_field_value_id,
            entry_id: 0,
            entry_field_id: 0,
            value: new_value,
        },
    )
    .await
    .map_err(|err| {
        error!("update_entry_field_value ERROR {}", err.to_string());
        ()
    })?;

    debug!("update_entry_field_value SUCCESS");
    Ok(())
}
