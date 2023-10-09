use crate::{check_group_chat_exists, check_profile_exists};
use entities::*;
use log::*;
use sea_orm::*;

pub async fn insert_group_chat_member(
    profile_id: i32,
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<group_chat_member::Model, DbErr> {
    let target_profile = check_profile_exists(profile_id, connection).await;

    if target_profile.is_err() {
        warn!(
            "C: Unable to add a profile, which does not exist, to a group: {}",
            target_profile.as_ref().unwrap_err()
        );

        return Err(target_profile.unwrap_err());
    }

    let target_group_chat = check_group_chat_exists(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        warn!(
            "C: Unable to add a profile to a group chat, which does not exist: {}",
            target_group_chat.as_ref().unwrap_err()
        );

        return Err(target_group_chat.unwrap_err());
    }

    let new_member = group_chat_member::ActiveModel {
        profile_id: ActiveValue::Set(target_profile.unwrap().profile_id),
        group_chat_id: ActiveValue::Set(target_group_chat.unwrap().group_chat_id),
        ..Default::default()
    }
    .insert(connection)
    .await;

    info!(
        "C: The member '{:?}' has been added to group chat '{:?}'",
        profile_id, group_chat_id
    );

    return new_member;
}

pub async fn get_members_of_group(
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<Vec<group_chat_member::Model>, DbErr> {
    let target_group_chat = check_group_chat_exists(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        debug!("R: Group chat with ID {:?} does not exist", group_chat_id);
        return Err(target_group_chat.unwrap_err());
    }

    let members_result = target_group_chat
        .unwrap()
        .find_related(group_chat_member::Entity)
        .all(connection)
        .await;

    match members_result {
        Ok(members) => {
            debug!("R: Members of group chat have been found");
            return Ok(members);
        }
        Err(err) => {
            debug!("R: Group chat with ID {:?} does not exist", group_chat_id);
            return Err(err);
        }
    }
}

pub async fn get_memberships_of_profile(
    profile_id: i32,
    connection: &DbConn,
) -> Result<Vec<group_chat::Model>, DbErr> {
    let target_profile = check_profile_exists(profile_id, connection).await;

    if target_profile.is_err() {
        debug!("R: Profile with ID {:?} does not exist", profile_id);
        return Err(target_profile.unwrap_err());
    }

    let memberships_result = target_profile
        .unwrap()
        .find_related(group_chat_member::Entity)
        .all(connection)
        .await;

    let mut groups = Vec::new();

    match memberships_result {
        Ok(memberships) => {
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
        }
        Err(err) => {
            debug!("R: Profile with ID {:?} does not exist", profile_id);
            return Err(err);
        }
    };

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
        warn!(
            "D: A memberhsip of profile with ID {:?} in group chat with ID {:?} does not exist",
            profile_id, group_chat_id
        );

        return Err(DbErr::Custom(
            "Couldn't find a group chat with the specified identifier.".to_owned(),
        ));
    }

    let delete_result = target_membership.unwrap().delete(connection).await;

    match delete_result {
        Ok(deleted_membership) => {
            debug!(
                "D: Membership has been deleted: Profile {}, Group chat {}",
                profile_id, group_chat_id
            );

            return Ok(deleted_membership);
        }
        Err(err) => {
            warn!(
                "D: Couldn't delete the membership: Profile: {}, Group chat {}",
                profile_id, group_chat_id
            );

            return Err(err);
        }
    }
}

pub async fn delete_members_of_group(
    group_chat_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let target_group_chat = check_group_chat_exists(group_chat_id, connection).await;

    if target_group_chat.is_err() {
        warn!("D: Group chat with ID {} does not exist", group_chat_id);
        return Err(target_group_chat.unwrap_err());
    }

    let delete_result = group_chat_member::Entity::delete_many()
        .filter(group_chat_member::Column::GroupChatId.eq(group_chat_id))
        .exec(connection)
        .await;

    match delete_result {
        Ok(deleted_memberships) => {
            debug!(
                "D: Memberships of group chat {} have been deleted",
                group_chat_id
            );

            return Ok(deleted_memberships);
        }
        Err(err) => {
            warn!("D: Group chat with ID {} does not exist", group_chat_id);
            return Err(err);
        }
    }
}

pub async fn delete_memberships_of_profile(
    profile_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let target_profile = check_profile_exists(profile_id, connection).await;

    if target_profile.is_err() {
        warn!("D: Profile with ID {:?} does not exist", profile_id);
        return Err(target_profile.unwrap_err());
    }

    let delete_result = group_chat_member::Entity::delete_many()
        .filter(group_chat_member::Column::ProfileId.eq(profile_id))
        .exec(connection)
        .await;

    match delete_result {
        Ok(deleted_memberships) => {
            debug!("D: Memberships of profile {} have been deleted", profile_id);
            return Ok(deleted_memberships);
        }
        Err(err) => {
            warn!("D: Profile with ID {} does not exist", profile_id);
            return Err(err);
        }
    }
}
