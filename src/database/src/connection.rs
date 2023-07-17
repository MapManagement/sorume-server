use dotenvy::dotenv;
use std::env;
use sea_orm::*;

pub async fn connect_to_database() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    let connection_string = env::var("DATABASE_URL")
        .expect("Couldn't find DATBASE_URL in .env file.");

    let db_connection = Database::connect(connection_string).await?;

    Ok(db_connection)
}
