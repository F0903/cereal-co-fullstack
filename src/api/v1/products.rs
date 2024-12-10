use super::{
    api_response::{ApiResponse, MessageObject},
    api_result::ApiResult,
    models::FormProduct,
};
use entity::{product::Model as ProductModel, Product};
use rocket::{serde::json::Json, State};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};

#[get("/get_products")]
pub async fn get_products(db: &State<DatabaseConnection>) -> ApiResult<Vec<ProductModel>> {
    let products = Product::find()
        .all(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;
    ApiResponse::ok(products).into()
}

#[post("/add_product", format = "json", data = "<product>")]
pub async fn add_product(
    db: &State<DatabaseConnection>,
    product: Json<FormProduct>,
) -> ApiResult<MessageObject> {
    let product = product.into_inner();
    println!("{:?}", &product);

    let full_product = product.into_active_model();
    Product::insert(full_product)
        .exec(db.inner())
        .await
        .map_err(|_| ApiResponse::bad_request())?;

    ApiResponse::success().into()
}
