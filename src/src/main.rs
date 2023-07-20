use futures::executor::block_on;
use migration::{Migrator, MigratorTrait, SchemaManager};
use sea_orm::{ConnectionTrait, Database, DbErr, Statement};

const DATABASE_URL: &str = "mysql://jan:jan@localhost:3306";
const DATABASE_NAME: &str = "sorume";
const BASE_TABLES: [&'static str; 5] = [
    "profile",
    "private_message",
    "group_chat",
    "group_chat_message",
    "group_chat_member",
];

fn main() {
    block_on(run_migrations());
    api::run();
}

async fn run_migrations() -> Result<(), DbErr> {
    let mut db_connection = Database::connect(DATABASE_URL).await?;

    db_connection
        .execute(Statement::from_string(
            db_connection.get_database_backend(),
            format!("CREATE DATABASE IF NOT EXISTS {};", DATABASE_NAME),
        ))
        .await?;

    let db_url = format!("{}/{}", DATABASE_URL, DATABASE_NAME);
    db_connection = Database::connect(&db_url).await?;

    let schema_manager = SchemaManager::new(&db_connection);

    for base_table in BASE_TABLES {
        assert!(schema_manager.has_table(base_table).await?);
    }

    Migrator::refresh(&db_connection).await?;

    Ok(())
}
