use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_connection = manager.get_connection();

        db_connection
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS profile (
                    profile_id INT AUTO_INCREMENT PRIMARY KEY,
                    username VARCHAR(32) NOT NULL,
                    password VARCHAR(1024) NOT NULL,
                    email_address VARCHAR(128) NOT NULL,
                    join_datetime DATETIME NOT NULL,
                    profile_picture VARCHAR(64)
                )",
            )
            .await?;

        db_connection
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS private_message (
                    private_message_id INT AUTO_INCREMENT PRIMARY KEY,
                    sender_id INT NOT NULL,
                    recipient_id INT NOT NULL,
                    content VARCHAR(2048),
                    CONSTRAINT fk_sender
                        FOREIGN KEY(sender_id) REFERENCES profile(profile_id),
                    CONSTRAINT fk_recipient
                        FOREIGN KEY(recipient_id) REFERENCES profile(profile_id)
                )",
            )
            .await?;

        db_connection
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS group_chat (
                    group_chat_id INT AUTO_INCREMENT PRIMARY KEY,
                    creation_date DATETIME NOT NULL,
                    group_picture VARCHAR(64)
                )",
            )
            .await?;

        db_connection
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS group_chat_message (
                    message_id INT AUTO_INCREMENT PRIMARY KEY,
                    author_id INT NOT NULL,
                    send_time DATETIME NOT NULL,
                    content VARCHAR(2048),
                    chat_id INT NOT NULL,
                    CONSTRAINT fk_message_author
                        FOREIGN KEY(author_id) REFERENCES profile(profile_id),
                    CONSTRAINT fk_message_group_chat
                        FOREIGN KEY(chat_id) REFERENCES group_chat(group_chat_id)
                )",
            )
            .await?;

        db_connection
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS group_chat_member (
                    member_id INT AUTO_INCREMENT PRIMARY KEY,
                    profile_id INT NOT NULL,
                    group_chat_id INT NOT NULL,
                    CONSTRAINT fk_profile
                        FOREIGN KEY(profile_id) REFERENCES profile(profile_id),
                    CONSTRAINT fk_member_group_chat
                        FOREIGN KEY(group_chat_id) REFERENCES group_chat(group_chat_id)
                )",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_connection = manager.get_connection();

        db_connection
            .execute_unprepared("DROP TABLE IF EXISTS group_chat_member")
            .await?;

        db_connection
            .execute_unprepared("DROP TABLE IF EXISTS group_chat_message")
            .await?;

        db_connection
            .execute_unprepared("DROP TABLE IF EXISTS private_message")
            .await?;

        db_connection
            .execute_unprepared("DROP TABLE IF EXISTS group_chat")
            .await?;

        db_connection
            .execute_unprepared("DROP TABLE IF EXISTS profile")
            .await?;

        Ok(())
    }
}
