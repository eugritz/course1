use log::debug;
use sea_orm::DbConn;

use entity;
use service::DeckService;

#[tauri::command]
pub async fn get_all_decks(
    state: tauri::State<'_, DbConn>,
) -> Result<Vec<entity::decks::Model>, ()> {
    debug!("get_all_decks START");
    let result = DeckService::find_all_decks(state.inner())
        .await
        .or(Ok(vec![]));
    debug!("get_all_decks FINISH");
    return result;
}

#[tauri::command]
pub async fn create_deck(
    state: tauri::State<'_, DbConn>,
    deck_title: String,
) -> Result<entity::decks::Model, ()> {
    debug!("create_deck START");
    let result = DeckService::create_deck(
        state.inner(),
        entity::decks::Model {
            id: 0,
            name: deck_title,
        },
    )
    .await
    .map(|x| x.try_into().unwrap())
    .map_err(|_| ());
    debug!("create_deck FINISH");
    return result;
}

#[tauri::command]
pub async fn rename_deck(
    state: tauri::State<'_, DbConn>,
    deck_id: i32,
    new_deck_title: String,
) -> Result<entity::decks::Model, ()> {
    debug!("rename_deck START");
    let result = DeckService::update_deck_by_id(
        state.inner(),
        deck_id,
        entity::decks::Model {
            id: 0,
            name: new_deck_title,
        },
    )
    .await
    .map_err(|_| ());
    debug!("rename_deck FINISH");
    return result;
}

#[tauri::command]
pub async fn delete_deck(
    state: tauri::State<'_, DbConn>,
    deck_id: i32,
) -> Result<(), ()> {
    debug!("delete_deck START");
    DeckService::delete_deck(state.inner(), deck_id)
        .await
        .map_err(|_| ())?;
    debug!("delete_deck FINISH");
    Ok(())
}

#[tauri::command]
pub async fn last_deck(
    state: tauri::State<'_, DbConn>,
) -> Result<Option<entity::decks::Model>, ()> {
    debug!("last_deck START");
    let result = DeckService::find_last_deck(state.inner())
        .await
        .map(|x| x)
        .map_err(|_| ());
    debug!("last_deck FINISH");
    return result;
}
