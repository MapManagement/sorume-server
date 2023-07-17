use crate::{get_group_chat_by_id, check_group_chat_exists, check_profile_exists};
use entities::*;
use sea_orm::*;

pub async fn insert_group_chat_message(
    author_profile_id: i32,
    group_chat_id: i32,
    content: String,
    connection: &DbConn,
) -> Result<group_chat_message::Model, DbErr> {
    let target_profile = check_profile_exists(author_profile_id, connection).await;

    if target_profile.is_err() {
        return Err(target_profile.unwrap_err());
    }

    let target_group_chat = check_group_chat_exists(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        return Err(target_group_chat.unwrap_err());
    }

    let group_chat_id = target_group_chat.unwrap().group_chat_id.to_owned();
    let author_id = target_profile.unwrap().profile_id.to_owned();

    let new_message = group_chat_message::ActiveModel {
        author_id: ActiveValue::Set(author_id),
        chat_id: ActiveValue::Set(group_chat_id),
        content: ActiveValue::Set(Some(content.to_owned())),
        ..Default::default()
    }
    .insert(connection)
    .await;

    return new_message;
}

pub async fn get_group_message_by_id(
    message_id: i32,
    connection: &DbConn,
) -> Result<group_chat_message::Model, DbErr> {
    let target_message = group_chat_message::Entity::find_by_id(message_id)
        .one(connection)
        .await?
        .ok_or(DbErr::Query(
            "Couldn't find a group message with the specified identifier.".to_owned(),
        ));

    return target_message;
}

pub async fn get_messages_of_group(
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<Vec<group_chat_message::Model>, DbErr> {
    let target_group_chat = check_group_chat_exists(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        return Err(target_group_chat.unwrap_err());
    }

    Ok(target_group_chat
        .unwrap()
        .find_related(group_chat_message::Entity)
        .all(connection)
        .await?)
}

pub async fn get_group_messages_of_profile(
    profile_id: i32,
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<Vec<group_chat_message::Model>, DbErr> {
    let target_profile = check_profile_exists(profile_id, connection).await;

    if target_profile.is_err() {
        return Err(target_profile.unwrap_err());
    }

    let target_group_chat = check_group_chat_exists(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        return Err(target_group_chat.unwrap_err());
    }

    let group_chat_id = target_group_chat.unwrap().group_chat_id.to_owned();
    let author_id = target_profile.unwrap().profile_id.to_owned();

    let group_messages = group_chat_message::Entity::find()
        .filter(group_chat_message::Column::ChatId.eq(group_chat_id))
        .filter(group_chat_message::Column::AuthorId.eq(author_id))
        .all(connection)
        .await;

    return group_messages;
}

pub async fn update_group_message(
    message_id: i32,
    content: String,
    connection: &DbConn,
) -> Result<group_chat_message::Model, DbErr> {
    let target_message = get_group_message_by_id(message_id, connection).await;

    if target_message.is_err() {
        return Err(DbErr::Query(
            "Couldn't find a group message with the specified identifier.".to_owned(),
        ));
    }

    let mut target_message: group_chat_message::ActiveModel = target_message.unwrap().into();
    target_message.content = Set(Some(content.to_owned()));

    target_message.update(connection).await
}

pub async fn delete_single_group_message(
    group_chat_message_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let target_message = group_chat_message::Entity::find()
        .filter(group_chat_member::Column::ProfileId.eq(group_chat_message_id))
        .one(connection)
        .await?;

    if target_message.is_none() {
        return Err(DbErr::Query(
            "Couldn't find a group chat with the specified identifier.".to_owned(),
        ));
    }

    Ok(target_message.unwrap().delete(connection).await?)
}

pub async fn delete_messages_of_group(
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let target_group_chat = check_group_chat_exists(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        return Err(target_group_chat.unwrap_err());
    }

    let group_chat_id = target_group_chat.unwrap().group_chat_id.to_owned();

    Ok(group_chat_message::Entity::delete_many()
        .filter(group_chat_member::Column::GroupChatId.eq(group_chat_id))
        .exec(connection)
        .await?)
}

pub async fn delete_group_messages_of_profile(
    profile_id: i32,
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let target_profile = check_profile_exists(profile_id, connection).await;

    if target_profile.is_err() {
        return Err(target_profile.unwrap_err());
    }

    let target_group_chat = check_group_chat_exists(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        return Err(target_group_chat.unwrap_err());
    }

    let group_chat_id = target_group_chat.unwrap().group_chat_id.to_owned();
    let author_id = target_profile.unwrap().profile_id.to_owned();

    Ok(group_chat_message::Entity::delete_many()
        .filter(group_chat_member::Column::ProfileId.eq(author_id))
        .filter(group_chat_message::Column::ChatId.eq(group_chat_id))
        .exec(connection)
        .await?)
}

pub async fn check_group_message_exists(
    message_id: i32,
    connection: &DbConn,
) -> Result<group_chat_message::Model, DbErr> {
    let target_message = get_group_message_by_id(message_id, connection).await;

    if target_message.is_err() {
        return Err(DbErr::Query(
            "Couldn't find a group chat with the specified identifier.".to_owned(),
        ));
    }

    target_message
}
