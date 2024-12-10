use super::{
    api_response::{ApiResponse, MessageObject},
    api_result::ApiResult,
    models::FormProduct,
};
use entity::{product::Model as ProductModel, Product};
use rocket::{serde::json::Json, State};
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[get("/products")]
pub async fn get_products(db: &State<DatabaseConnection>) -> ApiResult<Vec<ProductModel>> {
    let products = Product::find()
        .all(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;
    println!("{:?}", products);

    ApiResponse::ok(products).into()
}

#[get("/products/<id>")]
pub async fn get_product(db: &State<DatabaseConnection>, id: i32) -> ApiResult<ProductModel> {
    let product = Product::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;
    println!("{:?}", product);

    // Test if the product actually exists.
    let product = product.ok_or(ApiResponse::bad_request())?;

    ApiResponse::ok(product).into()
}

#[post("/products", format = "json", data = "<product>")]
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

#[delete("/products/<id>")]
pub async fn delete_product(db: &State<DatabaseConnection>, id: i32) -> ApiResult<MessageObject> {
    let product = Product::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;
    println!("{:?}", &product);

    // Test if the product actually exists before deleting it.
    product.ok_or(ApiResponse::bad_request())?;

    Product::delete_by_id(id)
        .exec(db.inner())
        .await
        .map_err(|_| ApiResponse::internal_error())?;

    ApiResponse::success().into()
}

#[put("/products/<id>", format = "json", data = "<new_product>")]
pub async fn update_product(
    db: &State<DatabaseConnection>,
    id: i32,
    new_product: Json<FormProduct>,
) -> ApiResult<MessageObject> {
    let new_product = new_product.into_inner();
    println!("{:?}", &new_product);

    let mut full_new_product = new_product.into_active_model();
    // Set the id of the full product
    full_new_product.set(<entity::Product as EntityTrait>::Column::Id, id.into());

    Product::update(full_new_product)
        .filter(<entity::Product as EntityTrait>::Column::Id.eq(id))
        .exec(db.inner())
        .await
        .map_err(|_| ApiResponse::bad_request())?;

    ApiResponse::success().into()
}
