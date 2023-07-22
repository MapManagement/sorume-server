use crate::api_models::create::*;
use crate::AppState;
use actix_web::*;
use database::*;

#[post("/group_chat/{group_chat_id}/members")]
async fn new_group_chat_member(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
    new_group_chat_member: web::Json<NewGroupChatMember>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let insert_result = insert_group_chat_member(
        new_group_chat_member.profile_id,
        group_chat_id.to_owned(),
        &db_connection,
    )
    .await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::Ok().body("Error!"),
    }
}

#[get("/group_chat/{group_chat_id}/members")]
async fn get_all_group_chat_members(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_members_of_group(group_chat_id.to_owned(), db_connection).await;

    // TODO: another return schema
    match query_result {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified group chat!"),
    }
}

#[delete("/group_chat/{group_chat_id}/members")]
async fn delete_all_group_chat_members(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result = delete_members_of_group(group_chat_id.to_owned(), db_connection).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified group chat!"),
    }
}

#[delete("/group_chat/{group_chat_id}/members/{profile_id}")]
async fn delete_single_group_chat_member(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
    profile_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result = delete_single_membership(
        group_chat_id.to_owned(),
        profile_id.to_owned(),
        db_connection,
    )
    .await;

    // TODO: error text
    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::NotFound().body("Error!"),
    }
}

pub fn group_chat_members_config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_group_chat_member);
    cfg.service(get_all_group_chat_members);
    cfg.service(delete_all_group_chat_members);
    cfg.service(delete_single_group_chat_member);
}
