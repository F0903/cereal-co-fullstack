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
                    .col(pk_auto(Product::Id).unsigned())
                    .col(string_len(Product::Name, 255))
                    .col(text(Product::Description))
                    .col(string_len(Product::Manufacturer, 255))
                    .col(unsigned(Product::Quantity))
                    .col(decimal_len(Product::Price, 10, 2))
                    .col(string_null(Product::ImageUrl))
                    .col(json_binary(Product::Attributes))
                    .index(
                        Index::create()
                            .name("idx-name-manufacturer")
                            .table(Product::Table)
                            .col(Product::Name)
                            .col(Product::Manufacturer)
                            .unique(),
                    )
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
pub enum Product {
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
