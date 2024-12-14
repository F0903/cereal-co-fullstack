use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;

use crate::entities::{order, order_item};

#[derive(Serialize, Deserialize, Debug)]
pub struct FormOrderItem {
    pub product_id: i32,
    pub quantity: i32,
}

// The model to receive when submitting an order.
#[derive(Serialize, Deserialize, Debug)]
pub struct FormOrder {
    pub shipping_name: String,
    pub shipping_phone: String,
    pub shipping_mail: String,
    pub shipping_address: String,
    pub total: Decimal,
    pub order_items: Vec<FormOrderItem>,
}

// The model to send when getting an order.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInfo {
    pub order_form: FormOrder,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl OrderInfo {
    pub fn create_from_orm_model(
        order: order::Model,
        order_items: &Vec<order_item::Model>,
    ) -> Self {
        Self {
            order_form: FormOrder {
                shipping_name: order.shipping_name,
                shipping_address: order.shipping_address,
                shipping_mail: order.shipping_mail,
                shipping_phone: order.shipping_phone,
                order_items: order_items
                    .into_iter()
                    .map(|x| FormOrderItem {
                        product_id: x.product_id,
                        quantity: x.quantity,
                    })
                    .collect(),
                total: order.total,
            },
            created_at: order.created_at,
            updated_at: order.updated_at,
        }
    }
}
