use sea_orm::entity::prelude::Json;
use serde::Deserialize;
use sqlx::types::Decimal;

/// The model to be used for order form submission
#[derive(Deserialize, Debug)]
pub struct FormOrder {
    pub shipping_name: String,
    pub shipping_phone: String,
    pub shipping_mail: String,
    pub total: Decimal,
    pub products: Json,
}
