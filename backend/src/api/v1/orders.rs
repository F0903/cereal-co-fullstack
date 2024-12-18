use super::{
    api_response::{ApiResponse, IdObject},
    api_result::{ApiResult, ApiResultIntoError, ApiResultIntoOk},
    models::{OrderForm, OrderResponse},
};
use crate::{
    auth::JWT,
    entities::{order, order_item, user},
};
use rocket::{serde::json::Json, State};
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[post("/orders", format = "json", data = "<order_form>")]
pub async fn add_order(
    db: &State<DatabaseConnection>,
    order_form: Json<OrderForm>,
) -> ApiResult<IdObject> {
    // In a real world web store, there would be some sort of payment id check here.

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

    ApiResponse::ok(IdObject { id: order.id }).into_ok()
}

#[get("/orders/<id>")]
pub async fn get_order(
    jwt: JWT,
    db: &State<DatabaseConnection>,
    id: i32,
) -> ApiResult<OrderResponse> {
    let orders = order::Entity::find_by_id(id)
        .find_with_related(order_item::Entity)
        .all(db.inner())
        .await
        .map_err(|_| ApiResponse::bad_request())?;

    // It's guaranteed to only contain one order, since ID is unique.
    let (order, order_items) = &orders[0];

    let calling_user = user::Entity::find_by_id(jwt.claims.sub)
        .one(db.inner())
        .await
        .map_err(|_| ApiResponse::unauthorized())?
        .ok_or(ApiResponse::unauthorized())?;

    // Make sure the mail of the user and requested order is matching.
    if calling_user.mail != order.shipping_mail {
        return ApiResponse::unauthorized().into_error();
    }

    let order_info = OrderResponse::create_from_orm_model(order.clone(), order_items);

    ApiResponse::ok(order_info).into_ok()
}

#[get("/orders?<mail>")]
pub async fn get_orders_by_mail(
    jwt: JWT,
    db: &State<DatabaseConnection>,
    mail: &str,
) -> ApiResult<Vec<OrderResponse>> {
    let calling_user = user::Entity::find_by_id(jwt.claims.sub)
        .one(db.inner())
        .await
        .map_err(|_| ApiResponse::unauthorized())?
        .ok_or(ApiResponse::unauthorized())?;

    // Make sure the mail of the user and requested order is matching.
    if calling_user.mail != mail {
        return ApiResponse::unauthorized().into_error();
    }

    let select = order::Entity::find()
        .find_with_related(order_item::Entity)
        .filter(order::Column::ShippingMail.eq(mail));

    let orders = select
        .all(db.inner())
        .await
        .map_err(|_| ApiResponse::bad_request())?;

    let order_infos = orders
        .into_iter()
        .map(|(order, order_items)| OrderResponse::create_from_orm_model(order, &order_items))
        .collect();

    ApiResponse::ok(order_infos).into_ok()
}
