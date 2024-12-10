use super::api_response::{ApiResponse, ApiResult};
use crate::entity::{prelude::Product, product::Model as ProductModel};
use rocket::State;
use sea_orm::{DatabaseConnection, EntityTrait};

#[get("/get_products")]
pub async fn get_products(db: &State<DatabaseConnection>) -> ApiResult<Vec<ProductModel>> {
    let products = Product::find()
        .all(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;
    ApiResponse::ok(products).into()
}
