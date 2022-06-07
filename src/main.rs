#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::{Build, Rocket, routes};

mod common;
mod biz;
mod model;
mod service;

use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;

use common::health_controller;
use biz::contents::contents_controller;
use biz::template::bill_book_template_controller;
use biz::bill::bill_controller;


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
        .mount("/fortune/template", routes![
            bill_book_template_controller::list,
        ])
        .mount("/fortune/bill", routes![
            bill_controller::add,
        ])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
    
}

