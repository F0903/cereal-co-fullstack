#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

// SeaORM generated entities. Ignore warnings from this (things in the prelude I don't use)
#[allow(warnings)]
mod entities;

mod api;
mod auth;
mod setup;
mod utils;

use rocket::{
    fs::{FileServer, Options},
    Config,
};
use setup::set_up_db;
use utils::pass_config::PassConfig;

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .configure(Config {
            address: std::env::var("LISTEN_ADDR")
                .unwrap_or("127.0.0.1".to_owned())
                .parse()
                .expect("Could not parse listen IP address!"),
            ..Default::default()
        })
        .pass_config(api::config)
        .mount("/static", FileServer::new("static", Options::Index))
}
