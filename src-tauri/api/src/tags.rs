use log::{debug, error};
use sea_orm::{DbConn, DbErr};

use entity::tags;
use service::TagService;

#[tauri::command]
pub async fn get_all_tags(
    state: tauri::State<'_, DbConn>,
) -> Result<Vec<tags::Model>, ()> {
    debug!("get_all_tags CALL");
    let result = TagService::find_all_tags(state.inner())
        .await
        .map_err(|err| {
            error!("get_all_tags ERROR {}", err.to_string());
        })?;

    debug!("get_all_tags SUCCESS");
    Ok(result)
}
