use rocket::Route;

mod api_response;
mod api_result;
mod auth;
mod models;
mod orders;
mod products;

pub use api_response::{ApiError, ApiResponse};

pub fn get_routes() -> Vec<Route> {
    routes![
        products::add_product,
        products::get_product,
        products::get_products,
        products::update_product,
        products::delete_product,
        orders::add_order,
        orders::get_order,
        orders::get_orders_by_filter,
        auth::login,
        auth::logout,
        auth::signup,
    ]
}
