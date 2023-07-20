use crate::{check_group_chat_exists, check_profile_exists};
use entities::*;
use sea_orm::*;

pub async fn insert_group_chat_member(
    profile_id: i32,
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<group_chat_member::Model, DbErr> {
    let target_profile = check_profile_exists(profile_id, connection).await;

    if target_profile.is_err() {
        return Err(target_profile.unwrap_err());
    }

    let target_group_chat = check_group_chat_exists(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        return Err(target_group_chat.unwrap_err());
    }

    let new_member = group_chat_member::ActiveModel {
        profile_id: ActiveValue::Set(target_profile.unwrap().profile_id),
        group_chat_id: ActiveValue::Set(target_group_chat.unwrap().group_chat_id),
        ..Default::default()
    }
    .insert(connection)
    .await;

    return new_member;
}

pub async fn get_members_of_group(
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<Vec<group_chat_member::Model>, DbErr> {
    let target_group_chat = check_group_chat_exists(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        return Err(target_group_chat.unwrap_err());
    }

    Ok(target_group_chat
        .unwrap()
        .find_related(group_chat_member::Entity)
        .all(connection)
        .await?)
}

pub async fn get_memberships_of_profile(
    profile_id: i32,
    connection: &DbConn,
) -> Result<Vec<group_chat::Model>, DbErr> {
    let target_profile = check_profile_exists(profile_id, connection).await;

    if target_profile.is_err() {
        return Err(target_profile.unwrap_err());
    }

    let memberships = target_profile
        .unwrap()
        .find_related(group_chat_member::Entity)
        .all(connection)
        .await?;

    let mut groups = Vec::new();

    for membership in memberships {
        let group = membership
            .find_related(group_chat::Entity)
            .one(connection)
            .await?;

        if group.is_none() {
            continue;
        }

        groups.push(group.unwrap());
    }

    Ok(groups)
}

pub async fn delete_single_membership(
    group_chat_id: i32,
    profile_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let target_membership = group_chat_member::Entity::find()
        .filter(group_chat_member::Column::ProfileId.eq(profile_id))
        .filter(group_chat_member::Column::GroupChatId.eq(group_chat_id))
        .one(connection)
        .await?;

    if target_membership.is_none() {
        return Err(DbErr::Custom(
            "Couldn't find a group chat with the specified identifier.".to_owned(),
        ));
    }

    Ok(target_membership.unwrap().delete(connection).await?)
}

pub async fn delete_members_of_group(
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let target_group_chat = check_group_chat_exists(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        return Err(target_group_chat.unwrap_err());
    }

    Ok(group_chat_member::Entity::delete_many()
        .filter(group_chat_member::Column::GroupChatId.eq(group_chat_id))
        .exec(connection)
        .await?)
}

pub async fn delete_memberships_of_profile(
    profile_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let target_profile = check_profile_exists(profile_id, connection).await;

    if target_profile.is_err() {
        return Err(target_profile.unwrap_err());
    }

    Ok(group_chat_member::Entity::delete_many()
        .filter(group_chat_member::Column::ProfileId.eq(profile_id))
        .exec(connection)
        .await?)
}
