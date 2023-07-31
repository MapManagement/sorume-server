use crate::api_models::group_chat_message_schema::*;
use crate::AppState;
use actix_web::*;
use database::*;

/// Create new group chat
///
/// Create a group chat using the post data
#[utoipa::path(
    tag = "Group Chat Message",
    request_body = PostGroupChatMessage,
    params(
        ("group_chat_id", description = "Identifier of group chat")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 500, description = "Error!")
    )
)]
#[post("/group_chat/{group_chat_id}/messages/new")]
pub(super) async fn new_group_chat_message(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
    new_group_chat_message: web::Json<PostGroupChatMessage>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let insert_result = insert_group_chat_message(
        new_group_chat_message.author_id,
        group_chat_id.to_owned(),
        new_group_chat_message.content.to_owned().unwrap(),
        &db_connection,
    )
    .await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::InternalServerError().body("Error!"),
    }
}
// TODO: combine group id and message id as primary key?
/*#[get("/group_chat/{group_chat_i}/messages/{message_id}")]
async fn get_group_chat_messages(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
    message_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_group_message_by_id(message_id.to_owned(), db_connection).await;

    if query_result.is_err() {
        return HttpResponse::NotFound().body("Couldn't find the specified group chat!");
    };

    let mut messages: Vec<GetGroupChatMessage> = vec![];

    for message in query_result.unwrap() {
        let message_schema = GetGroupChatMessage {
            message_id: message.message_id,
            author_id: message.author_id,
            send_time: message.send_time,
            content: message.content,
            chat_id: message.chat_id,
        };

        messages.push(message_schema);
    }

    HttpResponse::Ok().json(messages)
}*/

/// Get all messages of group chat
///
/// Retrieve all chat messages of a specific group chat using its identifier
#[utoipa::path(
    tag = "Group Chat Message",
    params(
        ("group_chat_id", description = "Identifier of group chat")
    ),
    responses(
        (status = 201, body = [GetGroupChatMessage]),
        (status = 404, description = "Couldn't find the specified group chat!"),
    )
)]
#[get("/group_chat/{group_chat_id}/messages")]
pub(super) async fn get_all_group_chat_messages(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_messages_of_group(group_chat_id.to_owned(), db_connection).await;

    if query_result.is_err() {
        return HttpResponse::NotFound().body("Couldn't find the specified group chat!");
    };

    let mut messages: Vec<GetGroupChatMessage> = vec![];

    for message in query_result.unwrap() {
        let message_schema = GetGroupChatMessage {
            message_id: message.message_id,
            author_id: message.author_id,
            send_time: message.send_time,
            content: message.content,
            chat_id: message.chat_id,
        };

        messages.push(message_schema);
    }

    HttpResponse::Ok().json(messages)
}

/// Get all messages of a profile in a group chat
///
/// Retrieve all messages of a specific profile, in a specific group chat using their identifiers
#[utoipa::path(
    tag = "Group Chat Message",
    params(
        ("group_chat_id", description = "Identifier of group chat"),
        ("profile_id", description = "Identifier of profile")
    ),
    responses(
        (status = 201, body = [GetGroupChatMessage]),
        (status = 404, description = "Couldn't find the specified group chat or profile!"),
    )
)]
#[get("/group_chat/{group_chat_id}/members/{profile_id}/messages")]
pub(super) async fn get_member_group_chat_messages(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
    profile_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_group_messages_of_profile(
        group_chat_id.to_owned(),
        profile_id.to_owned(),
        db_connection,
    )
    .await;

    if query_result.is_err() {
        return HttpResponse::NotFound().body("Couldn't find the specified group chat or profile!");
    };

    let mut messages: Vec<GetGroupChatMessage> = vec![];

    for message in query_result.unwrap() {
        let message_schema = GetGroupChatMessage {
            message_id: message.message_id,
            author_id: message.author_id,
            send_time: message.send_time,
            content: message.content,
            chat_id: message.chat_id,
        };

        messages.push(message_schema);
    }

    HttpResponse::Ok().json(messages)
}

/// Update group chat message
///
/// Update a specific group chat message using its identifier and patch data
#[utoipa::path(
    tag = "Group Chat Message",
    request_body = PatchGroupChatMessage,
    params(
        ("group_chat_id", description = "Identifier of group chat"),
        ("group_chat_message_id", description = "Identifier of group chat message")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 404, description = "Couldn't find the specified group chat or message!"),
    )
)]
// TODO: affected by primary key change
#[patch("/group_chat/{group_chat_id}/messages/{message_id}")]
pub(super) async fn update_group_chat_message(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
    message_id: web::Path<i32>,
    updated_fields: web::Json<PatchGroupChatMessage>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let update_result = update_group_message(
        message_id.to_owned(),
        updated_fields.content.to_owned(),
        db_connection,
    )
    .await;

    match update_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => {
            HttpResponse::NotFound().body("Couldn't find the specified group chat or message!")
        }
    }
}

/// Delete message in a group chat
///
/// Delete a specific message in a specific group chat using their identifiers
#[utoipa::path(
    tag = "Group Chat Message",
    params(
        ("group_chat_id", description = "Identifier of group chat"),
        ("group_chat_message_id", description = "Identifier of group chat message")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 404, description = "Couldn't find the specified private message!"),
    )
)]
// TODO: affected by primary key change
#[delete("/group_chat/{group_chat_id}/message/{message_id}")]
pub(super) async fn delete_group_chat_message(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
    message_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result = delete_single_group_message(message_id.to_owned(), db_connection).await;

    // TODO: error text
    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => {
            HttpResponse::NotFound().body("Couldn't find the specified group chat or message!")
        }
    }
}

/// Delete all messages of group chat
///
/// Delete all messages of a specific group chat using its identifier
#[utoipa::path(
    tag = "Group Chat Message",
    params(
        ("group_chat_id", description = "Identifier of group chat")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 404, description = "Couldn't find the specified group chat!"),
    )
)]
#[delete("/group_chat/{group_chat_id}/messages")]
pub(super) async fn delete_all_group_chat_messages(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result = delete_messages_of_group(group_chat_id.to_owned(), db_connection).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified group chat!"),
    }
}

/// Delete all messages of profile in group chat
///
/// Delete all messages of a specific profile in a specific group chat using their identifiers
#[utoipa::path(
    tag = "Group Chat Message",
    params(
        ("group_chat_id", description = "Identifier of group chat"),
        ("profile_id", description = "Identifier of profile")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 404, description = "Couldn't find the specified group chat or profile!"),
    )
)]
#[delete("/group_chat/{group_chat_id}/members/{profile_id}/messages")]
pub(super) async fn delete_profile_group_chat_messages(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
    profile_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result = delete_group_messages_of_profile(
        profile_id.to_owned(),
        group_chat_id.to_owned(),
        db_connection,
    )
    .await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => {
            HttpResponse::NotFound().body("Couldn't find the specified group chat or profile!")
        }
    }
}

pub fn group_chat_message_config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_group_chat_message);
    cfg.service(get_all_group_chat_messages);
    cfg.service(get_member_group_chat_messages);
    cfg.service(update_group_chat_message);
    cfg.service(delete_group_chat_message);
    cfg.service(delete_all_group_chat_messages);
    cfg.service(delete_profile_group_chat_messages);
}
