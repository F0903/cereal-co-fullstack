use rocket::{Build, Rocket};

/// Utility for passing the builder to another function while retaining the builder chain.
pub trait PassConfig {
    fn pass_config(self, configurator: fn(Rocket<Build>) -> Rocket<Build>) -> Rocket<Build>;
}

impl PassConfig for Rocket<Build> {
    fn pass_config(self, configurator: fn(Rocket<Build>) -> Rocket<Build>) -> Rocket<Build> {
        configurator(self)
    }
}
