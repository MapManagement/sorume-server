use crate::api_models::group_chat_schema::*;
use crate::AppState;
use actix_web::*;
use database::*;

/// Create new group chat
///
/// Create a new group chat using post data
#[utoipa::path(
    tag = "Group Chat",
    request_body = PostGroupChat,
    responses(
        (status = 201, description = "Success!"),
        (status = 500, description = "Error!")
    )
)]
// TODO: group picture is not sent to database
#[post("/group_chat/new")]
pub(super) async fn new_group_chat(
    data: web::Data<AppState>,
    new_group_chat: web::Json<PostGroupChat>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let result = insert_group_chat(new_group_chat.member_ids.to_owned(), &db_connection).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::InternalServerError().body("Error!"),
    }
}

/// Get group chat
///
/// Get a specific group chat by its identifier
#[utoipa::path(
    tag = "Group Chat",
    params(
        ("group_chat_id", description = "Identifier of group_chat")
    ),
    responses(
        (status = 201, body = GetGroupChat),
        (status = 404, description = "Couldn't find the specified group chat!"),
    )
)]
#[get("/group_chat/{group_chat_id}")]
pub(super) async fn get_group_chat(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_group_chat_by_id(group_chat_id.to_owned(), &db_connection).await;

    match query_result {
        Ok(group_chat) => {
            let group_chat_schema = GetGroupChat {
                creation_date: group_chat.creation_date,
                group_picture: group_chat.group_picture,
            };

            HttpResponse::Ok().json(group_chat_schema)
        }
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified group chat!"),
    }
}

/// Update group chat
///
/// Update a specific group chat using its identifier and patch data
#[utoipa::path(
    tag = "Group Chat",
    request_body = PatchGroupChat,
    params(
        ("group_chat_id", description = "Identifier of group chat")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 404, description = "Couldn't find the specified group chat!"),
        (status = 500, description = "Failed!")
    )
)]
#[patch("/group_chat/{group_chat_id}")]
pub(super) async fn update_group_chat(
    data: web::Data<AppState>,
    updated_fields: web::Json<PatchGroupChat>,
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
                Err(_) => HttpResponse::InternalServerError().body("Failed!"),
            }
        }
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified group chat!"),
    }
}

/// Delete group chat
///
/// Delete a specific group chat by its identifier
#[utoipa::path(
    tag = "Group Chat",
    params(
        ("group_chat_id", description = "Identifier of group chat")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 404, description = "Couldn't find the specified group chat!"),
    )
)]
#[delete("/group_chat/delete/{group_chat_id}")]
pub(super) async fn delete_group_chat(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result = delete_group_chat_by_id(group_chat_id.to_owned(), &db_connection).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified group chat!"),
    }
}

pub fn group_chat_config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_group_chat);
    cfg.service(get_group_chat);
    cfg.service(update_group_chat);
    cfg.service(delete_group_chat);
}
