//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "group_chat")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub group_chat_id: i32,
    pub creation_date: DateTime,
    pub group_picture: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::group_chat_member::Entity")]
    GroupChatMember,
    #[sea_orm(has_many = "super::group_chat_message::Entity")]
    GroupChatMessage,
}

impl Related<super::group_chat_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupChatMember.def()
    }
}

impl Related<super::group_chat_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupChatMessage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
