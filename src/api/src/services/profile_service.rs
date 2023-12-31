use crate::api_models::profile_schema::*;
use crate::AppState;
use actix_web::*;
use database::sea_orm::DbErr;
use database::*;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Successful!")
}
/// Create new profile
///
/// Create a new platform profile using the post data
#[utoipa::path(
    tag = "Profile",
    request_body = PostProfile,
    responses(
        (status = 201, description = "Success!"),
        (status = 403, description = "Whitespaces cannot be used in usernames!"),
        (status = 500, description = "Error!")
    )
)]
#[post("/profile/new")]
pub(super) async fn new_profile(
    data: web::Data<AppState>,
    new_profile: web::Json<PostProfile>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    // TODO: hash password
    let result = insert_profile(
        &new_profile.username,
        &new_profile.displayname,
        &new_profile.password,
        &new_profile.email_address,
        &db_connection,
    )
    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("Success!"),
        Err(error) => match error {
            DbErr::Custom(text) => HttpResponse::Forbidden().body(text),
            _ => HttpResponse::InternalServerError().body("Error!"),
        },
    }
}

/// Get profile by id
///
/// Get a specific platform profile by its identifier
#[utoipa::path(
    tag = "Profile",
    params(
        ("profile_id", description = "Identifier of profile")
    ),
    responses(
        (status = 200, body = GetProfile),
        (status = 404, description = "Couldn't find the specified profile!")
    )
)]
#[get("/profile/{profile_id}")]
pub(super) async fn get_profile(
    data: web::Data<AppState>,
    profile_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_profile_by_id(profile_id.to_owned(), &db_connection).await;

    match query_result {
        Ok(profile) => {
            let profile_schema = GetProfile {
                username: profile.username,
                displayname: profile.displayname.unwrap(),
                email_address: profile.email_address,
                join_datetime: profile.join_datetime,
            };

            HttpResponse::Ok().json(profile_schema)
        }
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified profile!"),
    }
}

/// Get profile by username
///
/// Get a specific platform profile by its username
#[utoipa::path(
    tag = "Profile",
    params(
        ("username", description = "Username of profile")
    ),
    responses(
        (status = 200, body = GetProfile),
        (status = 404, description = "Couldn't find the specified profile!")
    )
)]
#[get("/profile/username/{profile_username}")]
pub(super) async fn get_profile_username(
    data: web::Data<AppState>,
    profile_username: web::Path<String>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_profile_by_username(&profile_username, &db_connection).await;

    match query_result {
        Ok(profile) => {
            let profile_schema = GetProfile {
                username: profile.username,
                displayname: profile.displayname.unwrap(),
                email_address: profile.email_address,
                join_datetime: profile.join_datetime,
            };

            HttpResponse::Ok().json(profile_schema)
        }
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified profile!"),
    }
}

/// Update profile
///
/// Update a specific platform profile by its identifier and patch data
#[utoipa::path(
    tag = "Profile",
    request_body = PatchProfile,
    params(
        ("profile_id", description = "Identifier of profile")
    ),
    responses(
        (status = 200, description = "Success!"),
        (status = 404, description = "Couldn't find the specified profile!")
    )
)]
#[patch("/profile/{profile_id}")]
pub(super) async fn update_profile(
    data: web::Data<AppState>,
    updated_fields: web::Json<PatchProfile>,
    profile_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_profile_by_id(profile_id.to_owned(), &db_connection).await;

    match query_result {
        Ok(profile) => {
            let update_result = database::update_profile(
                profile.profile_id,
                &updated_fields
                    .username
                    .to_owned()
                    .unwrap_or(profile.username),
                &updated_fields
                    .displayname
                    .to_owned()
                    .unwrap_or(profile.displayname.unwrap()),
                &updated_fields
                    .password
                    .to_owned()
                    .unwrap_or(profile.password),
                &updated_fields
                    .email_address
                    .to_owned()
                    .unwrap_or(profile.email_address),
                &updated_fields
                    .profile_picture
                    .to_owned()
                    .unwrap_or(profile.profile_picture.unwrap()),
                &db_connection,
            )
            .await;

            match update_result {
                Ok(_) => HttpResponse::Ok().body("Success!"),
                Err(_) => HttpResponse::NotFound().body("Failed!"),
            }
        }
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified profile!"),
    }
}

/// Delete profile
///
/// Delete s specific platform profile by its identifier
#[utoipa::path(
    tag = "Profile",
    params(
        ("profile_id", description = "Identifier of profile")
    ),
    responses(
        (status = 200, description = "Success!"),
        (status = 404, description = "Couldn't find the specified profile!")
    )
)]
#[delete("/profile/{profile_id}")]
pub(super) async fn delete_profile(
    data: web::Data<AppState>,
    profile_id: web::Path<i32>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result = delete_profile_by_id(profile_id.to_owned(), &db_connection).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified profile!"),
    }
}

pub fn profile_config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_profile);
    cfg.service(get_profile);
    cfg.service(get_profile_username);
    cfg.service(update_profile);
    cfg.service(delete_profile);
}
