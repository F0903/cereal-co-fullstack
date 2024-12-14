use super::m20241209_140447_create_product_table::Product;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Order table
        manager
            .create_table(timestamps(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(pk_auto(Order::Id))
                    .col(text(Order::ShippingName))
                    .col(text(Order::ShippingAddress))
                    .col(text(Order::ShippingPhone))
                    .col(text(Order::ShippingMail))
                    .col(decimal_len(Order::Total, 10, 2))
                    .to_owned(),
            ))
            .await?;

        // OrderItem table
        manager
            .create_table(
                Table::create()
                    .table(OrderItem::Table)
                    .if_not_exists()
                    .col(integer(OrderItem::OrderId))
                    .col(integer(OrderItem::ProductId))
                    .col(integer(OrderItem::Quantity))
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItem::Table, OrderItem::OrderId)
                            .to(Order::Table, Order::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItem::Table, OrderItem::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .primary_key(
                        Index::create()
                            .table(OrderItem::Table)
                            .col(OrderItem::OrderId)
                            .col(OrderItem::ProductId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    ShippingName,
    ShippingAddress,
    ShippingPhone,
    ShippingMail,
    Total,
}

#[derive(DeriveIden)]
enum OrderItem {
    Table,
    OrderId,
    ProductId,
    Quantity,
}
