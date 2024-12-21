use super::{
    api_response::{ApiResponse, IdObject},
    api_result::{ApiResult, ApiResultIntoError, ApiResultIntoOk},
    models::{OrderForm, OrderResponse},
};
use crate::{
    auth::JWT,
    entities::{order, order_item, product, user},
};
use rocket::{serde::json::Json, State};
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};

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

    let order_id = db
        .transaction(|txn| {
            // Async closures are not stabilized, so we have to do this
            Box::pin(async move {
                let order = order.insert(txn).await?;

                let items: Vec<order_item::ActiveModel> = order_form
                    .order_items
                    .iter()
                    .map(|x| order_item::ActiveModel {
                        order_id: Set(order.id),
                        product_id: Set(x.product_id),
                        quantity: Set(x.quantity),
                    })
                    .collect();

                for item in items {
                    let item = item.insert(txn).await?;
                    let product = product::Entity::find_by_id(item.product_id)
                        .lock_exclusive()
                        .one(txn)
                        .await?
                        .ok_or(DbErr::RecordNotFound(
                            "product with matching order item id could not be found".to_owned(),
                        ))?;

                    if let Some(val) = product.quantity.checked_sub(item.quantity) {
                        println!(
                            "subtracting product stock = {} - {} = {}",
                            product.quantity, item.quantity, val
                        );
                        // Update product quantity
                        let mut active_product = product.into_active_model();
                        active_product.quantity = Set(val);
                        active_product.update(txn).await?;
                    } else {
                        return Err(DbErr::Custom(
                            "item order quantity was greater than product stock quantity"
                                .to_owned(),
                        ));
                    }
                }

                Ok::<u32, DbErr>(order.id)
            })
        })
        .await?;

    ApiResponse::ok(IdObject { id: order_id }).into_ok()
}

#[get("/orders/<id>")]
pub async fn get_order(
    jwt: JWT,
    db: &State<DatabaseConnection>,
    id: u32,
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
