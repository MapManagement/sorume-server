use chrono::Local;
use entities::*;
use log::*;
use sea_orm::*;

// TODO: profile picture
pub async fn insert_profile(
    username: &str,
    displayname: &Option<String>,
    hashed_password: &str,
    email_address: &str,
    connection: &DbConn,
) -> Result<profile::ActiveModel, DbErr> {
    if !is_username_valid(&username) {
        return Err(DbErr::Custom(
            "Whitespaces cannot be used in usernames!".to_string(),
        ));
    }

    let new_profile = profile::ActiveModel {
        username: ActiveValue::Set(username.to_string()),
        displayname: ActiveValue::Set(displayname.to_owned()),
        password: ActiveValue::Set(hashed_password.to_string()),
        email_address: ActiveValue::Set(email_address.to_string()),
        join_datetime: ActiveValue::Set(Local::now().naive_local()),
        profile_picture: ActiveValue::Set(Some("default".to_owned())),
        ..Default::default()
    }
    .save(connection)
    .await;

    match new_profile {
        Ok(profile) => {
            info!("C: New profile has been created: {:?}", profile.profile_id);
            return Ok(profile);
        }
        Err(err) => {
            warn!("C: Unable to create a new profile: {}", err);
            return Err(err);
        }
    }
}

pub async fn update_profile(
    profile_id: i32,
    username: &str,
    displayname: &str,
    hashed_password: &str,
    email_address: &str,
    profile_picture: &str,
    connection: &DbConn,
) -> Result<profile::Model, DbErr> {
    let target_profile = check_profile_exists(profile_id, connection).await;

    if target_profile.is_err() {
        warn!("U: Profile with ID {:?} does not exist", profile_id);
        return Err(target_profile.unwrap_err());
    }

    let target_profile = target_profile.unwrap();

    let update_result = profile::ActiveModel {
        profile_id: ActiveValue::Set(target_profile.profile_id),
        username: ActiveValue::Set(username.to_string()),
        displayname: ActiveValue::Set(Some(displayname.to_string())),
        password: ActiveValue::Set(hashed_password.to_string()),
        email_address: ActiveValue::Set(email_address.to_string()),
        join_datetime: ActiveValue::Set(Local::now().naive_local()),
        profile_picture: ActiveValue::Set(Some(profile_picture.to_string())),
    }
    .update(connection)
    .await;

    match update_result {
        Ok(updated_profile) => {
            info!(
                "U: Profile has been updated: {:?}",
                updated_profile.profile_id
            );
            return Ok(updated_profile);
        }
        Err(err) => {
            warn!("U: Unable to update profile: {}", err);
            return Err(err);
        }
    }
}

pub async fn get_profile_by_id(
    profile_id: i32,
    connection: &DbConn,
) -> Result<profile::Model, DbErr> {
    let read_result = profile::Entity::find_by_id(profile_id)
        .one(connection)
        .await;

    match read_result {
        Ok(target_profile) => {
            debug!("R: Profile has been found: {:?}", profile_id);
            return Ok(target_profile.unwrap());
        }
        Err(err) => {
            debug!("R: Profile with ID {:?} does not exist", profile_id);
            return Err(err);
        }
    }
}

pub async fn get_profile_by_username(
    username: &str,
    connection: &DbConn,
) -> Result<profile::Model, DbErr> {
    let read_result = profile::Entity::find()
        .filter(profile::Column::Username.eq(username))
        .one(connection)
        .await;

    match read_result {
        Ok(target_profile) => {
            debug!("R: Profile has been found: {}", username);
            return Ok(target_profile.unwrap());
        }
        Err(err) => {
            debug!("R: Profile with username {:?} does not exist", username);
            return Err(err);
        }
    }
}

pub async fn delete_profile_by_id(
    profile_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let delete_result = profile::Entity::delete_by_id(profile_id)
        .exec(connection)
        .await;

    match delete_result {
        Ok(deleted_profile) => {
            debug!("D: Profile has been deleted: {}", profile_id);
            return Ok(deleted_profile);
        }
        Err(err) => {
            warn!("D: Profile with ID {:?} does not exist", profile_id);
            return Err(err);
        }
    }
}

pub async fn check_profile_exists(
    profile_id: i32,
    connection: &DbConn,
) -> Result<profile::Model, DbErr> {
    let target_profile = get_profile_by_id(profile_id, connection).await;

    if target_profile.is_err() {
        return Err(target_profile.unwrap_err());
    };

    return Ok(target_profile.unwrap());
}

fn is_username_valid(username: &str) -> bool {
    return !username.contains(char::is_whitespace) && username.len() < 33;
}
