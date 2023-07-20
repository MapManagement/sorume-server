use chrono::Local;
use entities::*;
use sea_orm::*;

// TODO: profile picture
pub async fn insert_profile(
    username: &str,
    hashed_password: &str,
    email_address: &str,
    connection: &DbConn,
) -> Result<profile::ActiveModel, DbErr> {
    let new_profile = profile::ActiveModel {
        username: ActiveValue::Set(username.to_string()),
        password: ActiveValue::Set(hashed_password.to_string()),
        email_address: ActiveValue::Set(email_address.to_string()),
        join_datetime: ActiveValue::Set(Local::now().naive_local()),
        profile_picture: ActiveValue::Set(Some("default".to_owned())),
        ..Default::default()
    }
    .save(connection)
    .await;

    return new_profile;
}

pub async fn update_profile(
    profile_id: i32,
    username: &str,
    hashed_password: &str,
    email_address: &str,
    profile_picture: &str,
    connection: &DbConn,
) -> Result<profile::Model, DbErr> {
    let target_profile = check_profile_exists(profile_id, connection).await;

    if target_profile.is_err() {
        return Err(target_profile.unwrap_err());
    }

    let target_profile = target_profile.unwrap();

    profile::ActiveModel {
        profile_id: ActiveValue::Set(target_profile.profile_id),
        username: ActiveValue::Set(username.to_string()),
        password: ActiveValue::Set(hashed_password.to_string()),
        email_address: ActiveValue::Set(email_address.to_string()),
        join_datetime: ActiveValue::Set(Local::now().naive_local()),
        profile_picture: ActiveValue::Set(Some(profile_picture.to_string())),
    }
    .update(connection)
    .await
}

pub async fn get_profile_by_id(
    profile_id: i32,
    connection: &DbConn,
) -> Result<profile::Model, DbErr> {
    let target_profile = profile::Entity::find_by_id(profile_id)
        .one(connection)
        .await?
        .ok_or(DbErr::Custom(
            "Couldn't find a profile with the specified identifier.".to_owned(),
        ));

    return target_profile;
}

pub async fn delete_profile_by_id(
    profile_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let target_profile = profile::Entity::delete_by_id(profile_id)
        .exec(connection)
        .await?;

    return Ok(target_profile);
}

pub async fn check_profile_exists(
    profile_id: i32,
    connection: &DbConn,
) -> Result<profile::Model, DbErr> {
    let target_profile = get_profile_by_id(profile_id, connection).await;

    if target_profile.is_err() {
        return Err(DbErr::Custom(
            "Couldn't find a profile with the specified identifier.".to_owned(),
        ));
    }

    target_profile
}
