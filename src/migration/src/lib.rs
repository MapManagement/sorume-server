pub use sea_orm_migration::prelude::*;

mod m20230511_212747_create_initial_tables;
mod m20230804_181132_add_displayname_to_profile;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230511_212747_create_initial_tables::Migration),
            Box::new(m20230804_181132_add_displayname_to_profile::Migration),
        ]
    }
}
