use crate::api_models::private_message_schema::*;
use crate::AppState;
use actix_web::*;
use database::*;

/// Create new private message
///
/// Create a new private message using post data
#[utoipa::path(
    tag = "Private Message",
    request_body = PostPrivateMessage,
    responses(
        (status = 201, description = "Success!"),
        (status = 500, description = "Error!")
    )
)]
#[post("/private_message/new")]
pub(super) async fn new_private_message(
    data: web::Data<AppState>,
    new_private_message: web::Json<PostPrivateMessage>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let result = insert_private_message(
        new_private_message.sender_id.to_owned(),
        new_private_message.recipient_id.to_owned(),
        new_private_message.content.to_owned(),
        &db_connection,
    )
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::InternalServerError().body("Error!"),
    }
}

/// Get private message
///
/// Get a specific private message by its identifier
#[utoipa::path(
    tag = "Private Message",
    params(
        ("private_message_id", description = "Identifier of private message")
    ),
    responses(
        (status = 201, body = GetPrivateMessage),
        (status = 404, description = "Couldn't find the specified private message!")
    )
)]
#[get("/private_message/{private_message_id}")]
pub(super) async fn get_private_message(
    data: web::Data<AppState>,
    private_message_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result =
        get_private_message_by_id(private_message_id.to_owned(), &db_connection).await;

    match query_result {
        Ok(private_message) => {
            let message_schema = GetPrivateMessage {
                sender_id: private_message.sender_id,
                recipient_id: private_message.recipient_id,
                content: private_message.content,
            };

            HttpResponse::Ok().json(message_schema)
        }
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified private message!"),
    }
}

/// Get all private messages of private chat
///
/// Retrieve all private messages of a specific private chat using post data
#[utoipa::path(
    tag = "Private Message",
    request_body = DeletePostPrivateChat,
    responses(
        (status = 201, body = [GetPrivateMessage]),
        (status = 404, description = "Couldn't find the specified private message!")
    )
)]
#[post("/private_message/chat")]
pub(super) async fn get_private_chat_messages(
    data: web::Data<AppState>,
    private_chat: web::Json<DeletePostPrivateChat>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_private_messages_of_chat(
        private_chat.sender_id.to_owned(),
        private_chat.recipient_id.to_owned(),
        &db_connection,
    )
    .await;

    if query_result.is_err() {
        return HttpResponse::NotFound().body("Couldn't find the specified private chat!");
    }

    let mut messages: Vec<GetPrivateMessage> = vec![];

    for message in query_result.unwrap() {
        let message_schema = GetPrivateMessage {
            sender_id: message.sender_id,
            recipient_id: message.recipient_id,
            content: message.content,
        };

        messages.push(message_schema);
    }

    HttpResponse::Ok().json(messages)
}

/// Update private message
///
/// Update a specific private message using its identifier and patch data
#[utoipa::path(
    tag = "Private Message",
    request_body = PatchPrivateMessage,
    params(
        ("private_message_id", description = "Identifier of private message")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 404, description = "Couldn't find the specified private message!"),
        (status = 500, description = "Failed!")
    )
)]
#[patch("/private_message/{private_message_id}")]
pub(super) async fn update_private_message(
    data: web::Data<AppState>,
    updated_fields: web::Json<PatchPrivateMessage>,
    private_message_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result =
        get_private_message_by_id(private_message_id.to_owned(), &db_connection).await;

    match query_result {
        Ok(private_message) => {
            let update_result = database::update_private_message(
                private_message.private_message_id,
                updated_fields.content.to_owned(),
                &db_connection,
            )
            .await;

            match update_result {
                Ok(_) => HttpResponse::Ok().body("Success!"),
                Err(_) => HttpResponse::InternalServerError().body("Failed!"),
            }
        }
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified private message!"),
    }
}

/// Delete private message
///
/// Delete a specific private message by its identifier
#[utoipa::path(
    tag = "Private Message",
    params(
        ("private_message_id", description = "Identifier of private message")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 500, description = "Error!")
    )
)]
#[delete("/private_message/delete/{private_message_id}")]
pub(super) async fn delete_private_message(
    data: web::Data<AppState>,
    private_message_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result =
        delete_private_message_by_id(private_message_id.to_owned(), &db_connection).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::InternalServerError().body("Error!"),
    }
}

/// Delete all private messages of private chat
///
/// Delete all private message of a specific private chat by its identifier
#[utoipa::path(
    tag = "Private Message",
    request_body = DeletePostPrivateChat,
    responses(
        (status = 201, description = "Success!"),
        (status = 500, description = "Error!")
    )
)]
#[delete("/private_message/chat/delete")]
pub(super) async fn delete_private_chat_messages(
    data: web::Data<AppState>,
    private_chat: web::Json<DeletePostPrivateChat>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result = delete_private_messages_of_chat(
        private_chat.sender_id.to_owned(),
        private_chat.recipient_id.to_owned(),
        &db_connection,
    )
    .await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::InternalServerError().body("Error!"),
    }
}

pub fn private_message_config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_private_message);
    cfg.service(get_private_message);
    cfg.service(get_private_chat_messages);
    cfg.service(update_private_message);
    cfg.service(delete_private_message);
    cfg.service(delete_private_chat_messages);
}
