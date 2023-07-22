use dotenvy::dotenv;
use sea_orm::*;
use std::env;

pub async fn connect_to_database(use_database: bool) -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    let db_user = env::var("MARIADB_USER").expect("Couldn't find MARIADB_USER.");
    let db_user_password =
        env::var("MARIADB_USER_PASSWORD").expect("Couldn't find MARIADB_USER_PASSWORD.");
    let db_database = env::var("MARIADB_DATABASE").expect("Couldn't find MARIADB_DATABASE.");
    let db_host = env::var("MARIADB_HOST").expect("Couldn't find MARIADB_HOST.");

    let mut connection_string = format!("mysql://{}:{}@{}", db_user, db_user_password, db_host);

    if use_database {
        connection_string = format!("{}/{}", connection_string, db_database);
    }

    let db_connection = Database::connect(connection_string).await?;

    Ok(db_connection)
}
