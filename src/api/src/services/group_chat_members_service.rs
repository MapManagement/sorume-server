use crate::api_models::group_chat_member_schema::*;
use crate::AppState;
use actix_web::*;
use database::*;

/// Add profile to group chat
///
/// Add a specific profile by post to an existing group chat
#[utoipa::path(
    tag = "Group Chat Member",
    request_body = PostGroupChatMember,
    params(
        ("group_chat_id", description = "Identifier of group chat")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 404, description = "Couldn't find the specified group chat!"),
    )
)]
#[post("/group_chat/{group_chat_id}/members/new")]
pub(super) async fn new_group_chat_member(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
    new_group_chat_member: web::Json<PostGroupChatMember>,
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
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified group chat!"),
    }
}

/// Get all profiles of group chat
///
/// Retrieve all profiles of a specific group chat using its identifier
#[utoipa::path(
    tag = "Group Chat Member",
    params(
        ("group_chat_id", description = "Identifier of group chat")
    ),
    responses(
        (status = 201, body = [GetGroupChatMember]),
        (status = 404, description = "Couldn't find the specified group chat!"),
    )
)]
#[get("/group_chat/{group_chat_id}/members")]
pub(super) async fn get_all_group_chat_members(
    data: web::Data<AppState>,
    group_chat_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_members_of_group(group_chat_id.to_owned(), db_connection).await;

    if query_result.is_err() {
        return HttpResponse::NotFound().body("Couldn't find the specified group chat!");
    }

    let mut group_members: Vec<GetGroupChatMember> = vec![];

    for member in query_result.unwrap() {
        let member_schema = GetGroupChatMember {
            profile_id: member.profile_id,
        };

        group_members.push(member_schema);
    }

    HttpResponse::Ok().json(group_members)
}

/// Remove all profiles from group chat
///
/// Remove all profiles of a specific group chat using its identifier
#[utoipa::path(
    tag = "Group Chat Member",
    params(
        ("group_chat_id", description = "Identifier of group chat")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 404, description = "Couldn't find the specified group chat!"),
    )
)]
#[delete("/group_chat/{group_chat_id}/members")]
pub(super) async fn delete_all_group_chat_members(
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

/// Remove single profile from group chat
///
/// Remove a specific profile from a given group chat using their identifiers
#[utoipa::path(
    tag = "Group Chat Member",
    params(
        ("group_chat_id", description = "Identifier of group chat"),
        ("profile_id", description = "Identifier of profile")
    ),
    responses(
        (status = 201, description = "Success!"),
        (status = 404, description = "Couldn't find the specified group chat or profile!"),
    )
)]
#[delete("/group_chat/{group_chat_id}/members/{profile_id}")]
pub(super) async fn delete_single_group_chat_member(
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

    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => {
            HttpResponse::NotFound().body("Couldn't find the specified group chat or profile!")
        }
    }
}

pub fn group_chat_members_config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_group_chat_member);
    cfg.service(get_all_group_chat_members);
    cfg.service(delete_all_group_chat_members);
    cfg.service(delete_single_group_chat_member);
}
