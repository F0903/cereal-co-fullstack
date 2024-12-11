use entity::product::ActiveModel;
use sea_orm::{entity::prelude::Json, DeriveIntoActiveModel};
use serde::Deserialize;
use sqlx::types::Decimal;

/// The model to be used for product form submission
#[derive(DeriveIntoActiveModel, Deserialize, Debug)]
pub struct FormProduct {
    pub name: String,
    pub description: String,
    pub manufacturer: String,
    pub quantity: i32,
    pub price: Decimal,
    pub attributes: Json,
}
