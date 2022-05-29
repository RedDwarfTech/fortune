#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, routes};

mod common;

use common::health_controller;

#[launch]
async fn rocket() -> _ {
    build_rocket()
}

fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/actuator", routes![
            health_controller::health,
            health_controller::liveness
        ])
}

