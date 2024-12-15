use ring::digest::{digest, SHA256};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(timestamps(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(boolean(User::IsAdmin))
                    .col(string_uniq(User::Username))
                    .col(string(User::PasswordHash))
                    .to_owned(),
            ))
            .await?;

        // Add default admin user
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(User::Table)
                    .columns([User::IsAdmin, User::Username, User::PasswordHash])
                    .values_panic([
                        true.into(),
                        "admin".into(),
                        hex::encode(digest(&SHA256, b"admin").as_ref()).into(),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    IsAdmin,
    Username,
    PasswordHash,
}
