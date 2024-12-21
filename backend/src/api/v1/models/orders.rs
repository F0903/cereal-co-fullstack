use sea_orm::prelude::{DateTimeUtc, Decimal};
use serde::{Deserialize, Serialize};

use crate::entities::{order, order_item};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderFormItem {
    pub product_id: u32,
    pub quantity: u32,
}

// The model to receive when submitting an order.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderForm {
    pub shipping_name: String,
    pub shipping_phone: String,
    pub shipping_mail: String,
    pub shipping_address: String,
    pub total: Decimal,
    pub order_items: Vec<OrderFormItem>,
}

// The model to send when getting an order.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderResponse {
    pub order_id: u32,
    pub order_form: OrderForm,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl OrderResponse {
    pub fn create_from_orm_model(
        order: order::Model,
        order_items: &Vec<order_item::Model>,
    ) -> Self {
        Self {
            order_id: order.id,
            order_form: OrderForm {
                shipping_name: order.shipping_name,
                shipping_address: order.shipping_address,
                shipping_mail: order.shipping_mail,
                shipping_phone: order.shipping_phone,
                order_items: order_items
                    .into_iter()
                    .map(|x| OrderFormItem {
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
