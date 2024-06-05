//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "entries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub entry_kind_id: i32,
    pub deck_id: i32,
    pub color_tag: i32,
    #[sea_orm(column_type = "Double")]
    pub progress: f64,
    pub created_at: Date,
    pub last_shown_at: Option<DateTimeUtc>,
    pub next_shown_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::decks::Entity",
        from = "Column::DeckId",
        to = "super::decks::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Decks,
    #[sea_orm(has_many = "super::entry_field_values::Entity")]
    EntryFieldValues,
    #[sea_orm(
        belongs_to = "super::entry_kinds::Entity",
        from = "Column::EntryKindId",
        to = "super::entry_kinds::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    EntryKinds,
    #[sea_orm(has_many = "super::entry_tags::Entity")]
    EntryTags,
}

impl Related<super::decks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Decks.def()
    }
}

impl Related<super::entry_field_values::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EntryFieldValues.def()
    }
}

impl Related<super::entry_kinds::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EntryKinds.def()
    }
}

impl Related<super::entry_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EntryTags.def()
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        super::entry_tags::Relation::Tags.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::entry_tags::Relation::Entries.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
