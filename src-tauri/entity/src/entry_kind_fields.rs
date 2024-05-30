//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,
)]
#[sea_orm(table_name = "entry_kind_fields")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub entry_kind_id: i32,
    pub order: i32,
    pub name: String,
    pub desc: Option<String>,
    pub r#type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::entry_kind_default_field::Entity")]
    EntryKindDefaultField,
    #[sea_orm(
        belongs_to = "super::entry_kinds::Entity",
        from = "Column::EntryKindId",
        to = "super::entry_kinds::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    EntryKinds,
}

impl Related<super::entry_kind_default_field::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EntryKindDefaultField.def()
    }
}

impl Related<super::entry_kinds::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EntryKinds.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
