#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

mod api;
mod entity;
mod setup;
mod utils;

use setup::set_up_db;
use utils::pass_config::PassConfig;

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build().manage(db).pass_config(api::config)
}
