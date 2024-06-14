use log::{debug, error};
use sea_orm::DbConn;

use entity::tags;
use service::TagService;

#[tauri::command]
pub async fn get_all_tags(
    state: tauri::State<'_, DbConn>,
) -> Result<Vec<tags::Model>, ()> {
    debug!("get_all_tags CALL");
    let result =
        TagService::find_all_tags(state.inner())
            .await
            .map_err(|err| {
                error!("get_all_tags ERROR {}", err.to_string());
                ()
            })?;

    debug!("get_all_tags SUCCESS");
    Ok(result)
}

#[tauri::command]
pub async fn rename_tag(
    state: tauri::State<'_, DbConn>,
    tag: String,
    tag_new_name: String,
) -> Result<(), ()> {
    debug!("rename_tag CALL");
    TagService::rename_tag(state.inner(), tag, tag_new_name)
        .await
        .map_err(|err| {
            error!("rename_tag ERROR {}", err.to_string());
            ()
        })?;

    debug!("rename_tag SUCCESS");
    Ok(())
}

#[tauri::command]
pub async fn delete_tag(
    state: tauri::State<'_, DbConn>,
    tag: String,
) -> Result<(), ()> {
    debug!("delete_tag CALL");
    TagService::delete_tag(state.inner(), tag)
        .await
        .map_err(|err| {
            error!("delete_tag ERROR {}", err.to_string());
            ()
        })?;

    debug!("delete_tag SUCCESS");
    Ok(())
}
