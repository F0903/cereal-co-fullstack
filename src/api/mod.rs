use rocket::{Build, Rocket};

mod v1;

pub fn config(builder: Rocket<Build>) -> Rocket<Build> {
    builder.mount("/api/v1/", v1::get_routes())
}
