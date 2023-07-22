mod api_models;
mod services;

use actix_web::*;
use database::sea_orm::DatabaseConnection;
use database::*;
use services::*;

struct AppState {
    db_connection: DatabaseConnection,
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let db_connection = connect_to_database(true).await.unwrap();
    let data = web::Data::new(AppState { db_connection });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(profile_service::profile_config)
            .configure(group_chat_service::group_chat_config)
            .configure(private_message_service::private_message_config)
            .configure(group_chat_members_service::group_chat_members_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
