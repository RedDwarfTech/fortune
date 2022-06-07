#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::{Build, Rocket, routes};

mod common;
mod biz;
mod model;
mod service;

use common::health_controller;
use biz::contents::contents_controller;
use biz::template::bill_book_template_controller;

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
        .mount("/fortune/contents", routes![
            contents_controller::tree,
            contents_controller::fetch_available_contents
        ])
        .mount("/fortune/template",routes![
            bill_book_template_controller::list
        ])
}

