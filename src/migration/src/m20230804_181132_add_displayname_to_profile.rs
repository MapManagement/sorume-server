use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_connection = manager.get_connection();

        db_connection
            .execute_unprepared(
                "ALTER TABLE profile 
                    ADD displayname varchar(32)
                        AFTER username",
            )
            .await?;

        db_connection
            .execute_unprepared(
                "ALTER TABLE profile 
                    MODIFY username varchar(32) NOT NULL",
            )
            .await?;

        db_connection
            .execute_unprepared(
                "ALTER TABLE profile 
                    ADD CONSTRAINT unique_username UNIQUE(username)",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_connection = manager.get_connection();

        db_connection
            .execute_unprepared(
                "ALTER TABLE profile 
                    DROP COLUMN IF EXISTS displayname",
            )
            .await?;

        db_connection
            .execute_unprepared(
                "ALTER TABLE profile 
                    MODIFY username varchar(32) NULL",
            )
            .await?;

        db_connection
            .execute_unprepared(
                "ALTER TABLE profile 
                    DROP CONSTRAINT IF EXISTS unique_username",
            )
            .await?;

        Ok(())
    }
}
