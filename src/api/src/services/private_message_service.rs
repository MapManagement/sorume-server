use crate::api_models::private_message_schema::*;
use crate::AppState;
use actix_web::*;
use database::*;

#[post("/private_message/new")]
async fn new_private_message(
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

#[get("/private_message/{private_message_id}")]
async fn get_private_message(
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

#[post("/private_message/chat")]
async fn get_private_chat_messages(
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

    match query_result {
        Ok(private_messages) => HttpResponse::Ok().json(private_messages),
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified private chat!"),
    }
}

#[patch("/private_message/{private_message_id}")]
async fn update_private_message(
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
                Err(_) => HttpResponse::NotFound().body("Failed!"),
            }
        }
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified private message!"),
    }
}

#[delete("/private_message/delete/{private_message_id}")]
async fn delete_private_message(
    data: web::Data<AppState>,
    private_message_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result =
        delete_private_message_by_id(private_message_id.to_owned(), &db_connection).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::Ok().body("Error!"),
    }
}

#[delete("/private_message/chat/delete")]
async fn delete_private_chat_messages(
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
        Err(_) => HttpResponse::Ok().body("Error!"),
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
