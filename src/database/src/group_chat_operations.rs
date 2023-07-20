use crate::{
    delete_members_of_group, get_profile_by_id,
    group_chat_member_operations::insert_group_chat_member,
};
use chrono::Local;
use entities::*;
use sea_orm::*;

// TODO: error handling
pub async fn insert_group_chat(
    member_ids: Vec<i32>,
    connection: &DbConn,
) -> Result<group_chat::Model, DbErr> {
    if member_ids.len() < 1 {
        return Err(DbErr::Custom(
            "A group chat needs at least one member.".to_owned(),
        ));
    }

    let new_group_chat = group_chat::ActiveModel {
        creation_date: ActiveValue::Set(Local::now().naive_local()),
        ..Default::default()
    }
    .insert(connection)
    .await
    .unwrap();

    let mut new_members = Vec::new();

    for member_id in member_ids {
        let member_profile = get_profile_by_id(member_id, connection).await;

        if member_profile.is_err() {
            continue;
        }

        let group_chat_id = new_group_chat.group_chat_id;

        let new_member = insert_group_chat_member(
            member_profile.unwrap().profile_id,
            group_chat_id,
            connection,
        )
        .await;

        new_members.push(new_member.unwrap());
    }

    return Ok(new_group_chat);
}

pub async fn update_group_chat(
    group_chat_id: i32,
    group_picture: String,
    connection: &DbConn,
) -> Result<group_chat::Model, DbErr> {
    let target_group_chat = get_group_chat_by_id(group_chat_id, connection).await;

    match target_group_chat {
        Ok(target_group_chat) => {
            let mut target_group_chat: group_chat::ActiveModel = target_group_chat.into();
            target_group_chat.group_picture = Set(Some(group_picture));

            Ok(target_group_chat.update(connection).await?)
        }
        _ => Err(DbErr::Custom(
            "Couldn't find a group chat with the specified identifier.".to_owned(),
        )),
    }
}

pub async fn get_group_chat_by_id(
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<group_chat::Model, DbErr> {
    let target_group_chat = group_chat::Entity::find_by_id(group_chat_id)
        .one(connection)
        .await?
        .ok_or(DbErr::Custom(
            "Couldn't find a group chat with the specified identifier.".to_owned(),
        ));

    return target_group_chat;
}

pub async fn delete_group_chat_by_id(
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let delete_result = delete_members_of_group(group_chat_id, connection).await;

    if delete_result.is_err() {
        return Err(DbErr::Custom(
            "Couldn't delete any group chat members.".to_owned(),
        ));
    }

    let target_group_chat = group_chat::Entity::delete_by_id(group_chat_id)
        .exec(connection)
        .await?;

    return Ok(target_group_chat);
}

pub async fn check_group_chat_exists(
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<group_chat::Model, DbErr> {
    let target_group_chat = get_group_chat_by_id(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        return Err(DbErr::Custom(
            "Couldn't find a group chat with the specified identifier.".to_owned(),
        ));
    }

    target_group_chat
}
