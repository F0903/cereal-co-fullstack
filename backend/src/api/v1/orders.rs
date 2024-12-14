use super::{
    api_response::{ApiResponse, MessageObject},
    api_result::ApiResult,
    models::{FormOrder, FormProduct},
};
use rocket::{serde::json::Json, State};
use sea_orm::{entity::*, query::*, DatabaseConnection};

#[post("/orders", format = "json", data = "<order>")]
pub async fn add_order(
    db: &State<DatabaseConnection>,
    order: Json<FormOrder>,
) -> ApiResult<MessageObject> {
    //TODO
    ApiResponse::success().into()
}
