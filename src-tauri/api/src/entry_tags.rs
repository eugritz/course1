use log::{debug, error};
use sea_orm::DbConn;

use service::EntryTagsService;

#[tauri::command]
pub async fn get_entry_tags(
    state: tauri::State<'_, DbConn>,
    entry_id: i32,
) -> Result<Vec<String>, ()> {
    debug!("get_entry_tags CALL");
    let result =
        EntryTagsService::get_entry_tags(state.inner(), entry_id)
            .await
            .map_err(|err| {
                error!("get_entry_tags ERROR {}", err.to_string());
                ()
            })?;

    debug!("get_entry_tags SUCCESS");
    Ok(result)
}
