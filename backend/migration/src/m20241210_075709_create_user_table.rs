use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
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
                    .col(string_uniq(User::Mail))
                    .col(string(User::PasswordHash))
                    .to_owned(),
            ))
            .await?;

        let argon2 = Argon2::default();
        let salt = SaltString::generate(OsRng);

        // Add default admin user
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(User::Table)
                    .columns([User::IsAdmin, User::Mail, User::PasswordHash])
                    .values_panic([
                        true.into(),
                        "admin@admin.com".into(),
                        argon2
                            .hash_password(b"admin", &salt)
                            .map_err(|_| DbErr::Custom("could not hash password".into()))?
                            .to_string()
                            .into(),
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
    Mail,
    PasswordHash,
}
