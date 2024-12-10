use rocket::Route;

mod api_response;
mod api_result;
mod models;
mod products;

//TODO
//mod auth;

pub fn get_routes() -> Vec<Route> {
    routes![
        products::add_product,
        products::get_product,
        products::get_products,
        products::update_product,
        products::delete_product
    ]
}
