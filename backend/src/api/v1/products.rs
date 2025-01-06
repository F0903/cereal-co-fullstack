use super::{
    api_response::{ApiResponse, MessageObject},
    api_result::{ApiResult, ApiResultIntoOk},
    models::ProductForm,
};
use crate::{auth::JWT, entities::product};
use rocket::{serde::json::Json, State};
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[get("/products")]
pub async fn get_products(db: &State<DatabaseConnection>) -> ApiResult<Vec<product::Model>> {
    let products = product::Entity::find()
        .all(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;

    ApiResponse::ok(products).into_ok()
}

#[get("/products/<id>")]
pub async fn get_product(db: &State<DatabaseConnection>, id: u32) -> ApiResult<product::Model> {
    let product = product::Entity::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;

    // Test if the product actually exists.
    let product = product.ok_or(ApiResponse::bad_request())?;

    ApiResponse::ok(product).into_ok()
}

#[post("/products", format = "json", data = "<product>")]
pub async fn add_product(
    jwt: JWT,
    db: &State<DatabaseConnection>,
    product: Json<ProductForm>,
) -> ApiResult<product::Model> {
    jwt.assert_admin()?;

    let active_product = product.into_inner().into_active_model();
    let result = product::Entity::insert(active_product)
        .exec(db.inner())
        .await
        .map_err(|_| ApiResponse::bad_request())?;

    let id = result.last_insert_id;
    let product = product::Entity::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?
        .ok_or(ApiResponse::internal_error())?;

    ApiResponse::ok(product).into_ok()
}

#[delete("/products/<id>")]
pub async fn delete_product(
    jwt: JWT,
    db: &State<DatabaseConnection>,
    id: u32,
) -> ApiResult<MessageObject> {
    jwt.assert_admin()?;

    let product = product::Entity::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;

    // Test if the product actually exists before deleting it.
    product.ok_or(ApiResponse::bad_request())?;

    product::Entity::delete_by_id(id)
        .exec(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;

    ApiResponse::success().into_ok()
}

#[put("/products/<id>", format = "json", data = "<new_product>")]
pub async fn update_product(
    jwt: JWT,
    db: &State<DatabaseConnection>,
    id: u32,
    new_product: Json<ProductForm>,
) -> ApiResult<MessageObject> {
    jwt.assert_admin()?;

    let mut active_new_product = new_product.into_inner().into_active_model();
    active_new_product.set(product::Column::Id, id.into());

    product::Entity::update(active_new_product)
        .filter(product::Column::Id.eq(id))
        .exec(db.inner())
        .await
        .map_err(|_| ApiResponse::bad_request())?;

    ApiResponse::success().into_ok()
}
