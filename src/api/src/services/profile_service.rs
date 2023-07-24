use crate::api_models::profile_schema::*;
use crate::AppState;
use actix_web::*;
use database::*;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Successful!")
}

#[post("/profile/new")]
async fn new_profile(
    data: web::Data<AppState>,
    new_profile: web::Json<PostProfile>,
) -> impl Responder {
    let db_connection = &data.db_connection;

    // TODO: hash password
    let result = insert_profile(
        &new_profile.username,
        &new_profile.password,
        &new_profile.email_address,
        &db_connection,
    )
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::InternalServerError().body("Error!"),
    }
}

#[get("/profile/{profile_id}")]
async fn get_profile(data: web::Data<AppState>, profile_id: web::Path<i32>) -> impl Responder {
    let db_connection = &data.db_connection;

    let query_result = get_profile_by_id(profile_id.to_owned(), &db_connection).await;

    match query_result {
        Ok(profile) => {
            let profile_schema = GetProfile {
                username: profile.username,
                email_address: profile.email_address,
                join_datetime: profile.join_datetime,
            };

            HttpResponse::Ok().json(profile_schema)
        }
        Err(_) => HttpResponse::NotFound().body("Couldn't find the specified profile!"),
    }
}

#[patch("/profile/{profile_id}")]
async fn update_profile(
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

#[delete("/profile/delete/{profile_id}")]
async fn delete_profile(data: web::Data<AppState>, profile_id: web::Path<i32>) -> impl Responder {
    let db_connection = &data.db_connection;

    let delete_result = delete_profile_by_id(profile_id.to_owned(), &db_connection).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().body("Success!"),
        Err(_) => HttpResponse::Ok().body("Error!"),
    }
}

pub fn profile_config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_profile);
    cfg.service(get_profile);
    cfg.service(update_profile);
    cfg.service(delete_profile);
}
