use crate::api_models::create::*;
use crate::api_models::update::*;
use crate::AppState;
use actix_web::*;
use database::*;

// TODO: group picture is not sent to database
#[post("/group_chat/new")]
async fn new_group_chat(
    data: web::Data<AppState>,
    new_group_chat: web::Json<NewGroupChatSchema>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let result = insert_group_chat(new_group_chat.member_ids.to_owned(), &db_connection).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::InternalServerError().body("Error!"),
    }
}

#[get("/group_chat/{group_chat_id}")]
async fn get_group_chat(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_group_chat_by_id(group_chat_id.to_owned(), &db_connection).await;

    match query_result {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified group chat!"),
    }
}

#[patch("/group_chat/{group_chat_id}")]
async fn update_group_chat(
    data: web::Data<AppState>,
    updated_fields: web::Json<UpdateGroupChatSchema>,
    group_chat_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_group_chat_by_id(group_chat_id.to_owned(), &db_connection).await;

    match query_result {
        Ok(group_chat) => {
            let update_result = database::update_group_chat(
                group_chat.group_chat_id,
                updated_fields.group_picture.to_owned(),
                &db_connection,
            )
            .await;

            match update_result {
                Ok(_) => HttpResponse::Ok().body("Success!"),
                Err(_) => HttpResponse::NotFound().body("Failed!"),
            }
        }
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified group chat!"),
    }
}

#[delete("/group_chat/delete/{group_chat_id}")]
async fn delete_group_chat(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result = delete_group_chat_by_id(group_chat_id.to_owned(), &db_connection).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::Ok().body("Error!"),
    }
}

pub fn group_chat_config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_group_chat);
    cfg.service(get_group_chat);
    cfg.service(update_group_chat);
    cfg.service(delete_group_chat);
}
