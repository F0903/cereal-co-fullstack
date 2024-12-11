use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(timestamps(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(pk_auto(Product::Id))
                    .col(string_len(Product::Name, 255))
                    .col(string_len(Product::Description, 255))
                    .col(string_len(Product::Manufacturer, 255))
                    .col(integer(Product::Quantity))
                    .col(decimal_len(Product::Price, 10, 2))
                    .col(string_null(Product::ImageUrl))
                    .col(json_binary(Product::Attributes))
                    .to_owned(),
            ))
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
    Description,
    Manufacturer,
    Quantity,
    Price,
    ImageUrl,
    Attributes,
}
