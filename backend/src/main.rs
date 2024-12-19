#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

mod api;
mod auth;
mod entities;
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
