use entities::*;
use log::*;
use sea_orm::*;

pub async fn insert_private_message(
    sender_id: i32,
    recipient_id: i32,
    content: Option<String>,
    connection: &DbConn,
) -> Result<private_message::Model, DbErr> {
    let new_message = private_message::ActiveModel {
        sender_id: ActiveValue::Set(sender_id),
        recipient_id: ActiveValue::Set(recipient_id),
        content: ActiveValue::Set(content.to_owned()),
        ..Default::default()
    }
    .insert(connection)
    .await;

    match new_message {
        Ok(message) => {
            info!(
                "C: New private message has been created: {:?}",
                message.private_message_id
            );
            return Ok(message);
        }
        Err(err) => {
            warn!("C: Unable to create a new private message: {}", err);
            return Err(err);
        }
    }
}

pub async fn update_private_message(
    message_id: i32,
    content: String,
    connection: &DbConn,
) -> Result<private_message::Model, DbErr> {
    let target_message = get_private_message_by_id(message_id, connection).await;

    if target_message.is_err() {
        warn!("U: Private message with ID {:?} does not exist", message_id);
        return Err(target_message.unwrap_err());
    }

    let mut target_message: private_message::ActiveModel = target_message.unwrap().into();
    target_message.content = Set(Some(content.to_owned()));

    let update_result = target_message.update(connection).await;

    match update_result {
        Ok(updated_message) => {
            info!(
                "U: Message has been updated: {:?}",
                updated_message.private_message_id
            );
            return Ok(updated_message);
        }
        Err(err) => {
            warn!("U: Unable to update profile: {}", err);
            return Err(err);
        }
    }
}

pub async fn get_private_messages_of_chat(
    sender_id: i32,
    recipient_id: i32,
    connection: &DbConn,
) -> Result<Vec<private_message::Model>, DbErr> {
    let read_result = private_message::Entity::find()
        .filter(private_message::Column::SenderId.eq(sender_id))
        .filter(private_message::Column::RecipientId.eq(recipient_id))
        .all(connection)
        .await;

    match read_result {
        Ok(messages) => {
            debug!(
                "R: Messages of private chat between {} and {} have been found",
                sender_id, recipient_id
            );
            return Ok(messages);
        }
        Err(err) => {
            debug!(
                "R: Private chat between {} and {} does not exist",
                sender_id, recipient_id
            );
            return Err(err);
        }
    }
}

pub async fn get_private_message_by_id(
    message_id: i32,
    connection: &DbConn,
) -> Result<private_message::Model, DbErr> {
    let message_result = private_message::Entity::find_by_id(message_id)
        .one(connection)
        .await;

    match message_result {
        Ok(message) => {
            debug!("R: Private message with ID {} has been found", message_id);
            return Ok(message.unwrap());
        }
        Err(err) => {
            debug!("R: Private message with ID {} does not exist", message_id);
            return Err(err);
        }
    }
}

pub async fn delete_private_message_by_id(
    private_message_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let delete_result = private_message::Entity::delete_by_id(private_message_id)
        .exec(connection)
        .await?;

    return Ok(delete_result);
}

pub async fn delete_private_messages_of_chat(
    sender_id: i32,
    recipient_id: i32,
    connection: &DbConn,
) -> Result<DeleteResult, DbErr> {
    let delete_result = private_message::Entity::delete_many()
        .filter(private_message::Column::SenderId.eq(sender_id))
        .filter(private_message::Column::RecipientId.eq(recipient_id))
        .exec(connection)
        .await?;

    return Ok(delete_result);
}
