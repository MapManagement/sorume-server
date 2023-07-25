use crate::api_models::group_chat_message_schema::*;
use crate::AppState;
use actix_web::*;
use database::*;

#[post("/group_chat/{group_chat_id}/messages")]
async fn new_group_chat_message(
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
        Err(_) => HttpResponse::Ok().body("Error!"),
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

#[get("/group_chat/{group_chat_id}/messages")]
async fn get_all_group_chat_messages(
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

#[get("/group_chat/{group_chat_id}/members/{profile_id}/messages")]
async fn get_member_group_chat_messages(
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

// TODO: affected by primary key change
#[patch("/group_chat/{group_chat_id}/messages/{message_id}")]
async fn update_group_chat_message(
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
        Err(_) => HttpResponse::NotFound().body("Failed!"),
    }
}

// TODO: affected by primary key change
#[delete("/group_chat/{group_chat_id}/message/{message_id}")]
async fn delete_group_chat_message(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
    message_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result = delete_single_group_message(message_id.to_owned(), db_connection).await;

    // TODO: error text
    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::NotFound().body("Error!"),
    }
}

#[delete("/group_chat/{group_chat_id}/messages")]
async fn delete_all_group_chat_messages(
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

#[delete("/group_chat/{group_chat_id}/members/{profile_id}/messages")]
async fn delete_profile_group_chat_messages(
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

pub fn group_chat_members_config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_group_chat_message);
    cfg.service(get_all_group_chat_messages);
    cfg.service(get_member_group_chat_messages);
    cfg.service(update_group_chat_message);
    cfg.service(delete_group_chat_message);
    cfg.service(delete_all_group_chat_messages);
    cfg.service(delete_profile_group_chat_messages);
}
