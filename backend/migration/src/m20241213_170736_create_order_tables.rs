use super::m20241209_140447_create_product_table::Product;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(timestamps(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(pk_auto(Order::Id))
                    .col(integer(Order::Total))
                    .col(text(Order::ShippingName))
                    .col(text(Order::ShippingAddress))
                    .col(text(Order::ShippingPhone))
                    .col(text(Order::ShippingMail))
                    .col(integer(Order::OrderItemsId))
                    .to_owned(),
            ))
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrderItem::Table)
                    .if_not_exists()
                    .col(pk_auto(OrderItem::Id))
                    .col(integer(OrderItem::OrderId))
                    .col(integer(OrderItem::ProductId))
                    .col(integer(OrderItem::Quantity))
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItem::Table, OrderItem::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create many-to-many relationships
        // We have to do this after the tables have been created, or it will error.
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Order::Table, Order::OrderItemsId)
                    .to(OrderItem::Table, OrderItem::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(OrderItem::Table, OrderItem::OrderId)
                    .to(Order::Table, Order::Id)
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
    OrderItemsId,
}

#[derive(DeriveIden)]
enum OrderItem {
    Table,
    Id,
    OrderId,
    ProductId,
    Quantity,
}
