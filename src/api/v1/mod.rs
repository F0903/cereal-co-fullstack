use rocket::Route;

mod api_response;
mod api_result;
mod models;
mod products;

//TODO
//mod auth;

pub fn get_routes() -> Vec<Route> {
    routes![products::get_products, products::add_product]
}
