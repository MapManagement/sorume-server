use database::connect_to_database;
use futures::executor::block_on;
use migration::{Migrator, MigratorTrait, SchemaManager};
use sea_orm::DbErr;

const BASE_TABLES: [&'static str; 5] = [
    "profile",
    "private_message",
    "group_chat",
    "group_chat_message",
    "group_chat_member",
];

fn main() {
    let migration_result = block_on(run_migrations());

    if let Some(err) = migration_result.err() {
        println!("Migrations failed: {err}");
    }

    let api_result = api::run();

    if let Some(err) = api_result.err() {
        println!("API failed: {err}");
    }
}

async fn run_migrations() -> Result<(), DbErr> {
    let db_connection = connect_to_database(true).await?;

    let schema_manager = SchemaManager::new(&db_connection);
    Migrator::refresh(&db_connection).await?;

    for base_table in BASE_TABLES {
        assert!(schema_manager.has_table(base_table).await?);
    }

    Ok(())
}
