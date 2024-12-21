use crate::entities::product::ActiveModel;
use sea_orm::{
    entity::prelude::{Decimal, Json},
    DeriveIntoActiveModel,
};
use serde::Deserialize;

/// The model to be used for product form submission
#[derive(DeriveIntoActiveModel, Deserialize, Debug)]
pub struct ProductForm {
    pub name: String,
    pub description: String,
    pub manufacturer: String,
    pub quantity: u32,
    pub price: Decimal,
    pub image_url: String,
    pub attributes: Json,
}
