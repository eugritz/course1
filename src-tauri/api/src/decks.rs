use sea_orm::DbConn;

use entity;
use service::DeckService;

#[tauri::command]
pub async fn get_all_decks(
    state: tauri::State<'_, DbConn>,
) -> Result<Vec<entity::decks::Model>, ()> {
    DeckService::find_all_decks(&state).await.or(Ok(vec![]))
}

#[tauri::command]
pub async fn create_deck(
    state: tauri::State<'_, DbConn>,
    deck_title: String,
) -> Result<entity::decks::Model, ()> {
    DeckService::create_deck(
        &state,
        entity::decks::Model {
            id: 0,
            name: deck_title,
        },
    )
    .await
    .map(|x| x.try_into().unwrap())
    .map_err(|_| ())
}

#[tauri::command]
pub async fn rename_deck(
    state: tauri::State<'_, DbConn>,
    deck_id: i32,
    new_deck_title: String,
) -> Result<entity::decks::Model, ()> {
    DeckService::update_deck_by_id(
        &state,
        deck_id,
        entity::decks::Model {
            id: 0,
            name: new_deck_title,
        },
    )
    .await
    .map(|x| x.try_into().unwrap())
    .map_err(|_| ())
}

#[tauri::command]
pub async fn delete_deck(
    state: tauri::State<'_, DbConn>,
    deck_id: i32,
) -> Result<(), ()> {
    DeckService::delete_deck(&state, deck_id)
        .await
        .map_err(|_| ())?;
    Ok(())
}
