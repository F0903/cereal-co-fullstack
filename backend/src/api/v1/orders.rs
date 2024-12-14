use super::{
    api_response::{ApiResponse, MessageObject},
    api_result::{ApiResult, ApiResultIntoError, ApiResultIntoOk},
    models::{FormOrder, OrderInfo},
};
use crate::entities::{order, order_item};
use rocket::{serde::json::Json, State};
use sea_orm::{entity::*, DatabaseConnection, QueryFilter};

#[post("/orders", format = "json", data = "<order_form>")]
pub async fn add_order(
    db: &State<DatabaseConnection>,
    order_form: Json<FormOrder>,
) -> ApiResult<MessageObject> {
    let order = order::ActiveModel {
        shipping_name: Set(order_form.shipping_name.clone()),
        shipping_address: Set(order_form.shipping_address.clone()),
        shipping_mail: Set(order_form.shipping_mail.clone()),
        shipping_phone: Set(order_form.shipping_phone.clone()),
        total: Set(order_form.total),
        ..Default::default()
    };
    println!("{:?}", order);
    let order = order
        .insert(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;
    println!("{:?}", order);

    let items: Vec<order_item::ActiveModel> = order_form
        .order_items
        .iter()
        .map(|x| order_item::ActiveModel {
            order_id: Set(order.id),
            product_id: Set(x.product_id),
            quantity: Set(x.quantity),
        })
        .collect();
    println!("{:?}", items);
    order_item::Entity::insert_many(items)
        .exec(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;

    ApiResponse::success().into_ok()
}

#[get("/orders/<id>")]
pub async fn get_order(db: &State<DatabaseConnection>, id: i32) -> ApiResult<OrderInfo> {
    let orders = order::Entity::find_by_id(id)
        .find_with_related(order_item::Entity)
        .all(db.inner())
        .await
        .map_err(|_| ApiResponse::bad_request())?;

    // It's guaranteed to only contain one order, since ID is unique.
    let (order, order_items) = &orders[0];
    let order_info = OrderInfo::create_from_orm_model(order.clone(), order_items);

    ApiResponse::ok(order_info).into_ok()
}

#[get("/orders?<mail>&<name>")]
pub async fn get_orders_by_filter(
    db: &State<DatabaseConnection>,
    mail: Option<&str>,
    name: Option<&str>,
) -> ApiResult<Vec<OrderInfo>> {
    if mail.is_none() && name.is_none() {
        return ApiResponse::bad_request().into_error();
    }

    let mut select = order::Entity::find().find_with_related(order_item::Entity);

    if let Some(mail) = mail {
        select = select.filter(order::Column::ShippingMail.eq(mail));
    }

    if let Some(name) = name {
        select = select.filter(order::Column::ShippingName.eq(name));
    }

    let orders = select
        .all(db.inner())
        .await
        .map_err(|_| ApiResponse::bad_request())?;

    let order_infos = orders
        .into_iter()
        .map(|(order, order_items)| OrderInfo::create_from_orm_model(order, &order_items))
        .collect();

    ApiResponse::ok(order_infos).into_ok()
}
