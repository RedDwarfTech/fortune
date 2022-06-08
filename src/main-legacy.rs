#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::{Build, Rocket, routes};
use rocket::serde::json::Json;

mod common;
mod biz;
mod model;
mod service;

use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use serde::{Deserialize, Serialize};

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
        .mount(
            "/",
            openapi_get_routes![
                bill_controller::get_all_users,
                bill_controller::add
            ],
        )
        .mount("/actuator", openapi_get_routes![
            health_controller::health,
            health_controller::liveness
        ])
        .mount("/fortune/contents", openapi_get_routes![
            contents_controller::tree,
            contents_controller::fetch_available_contents
        ])
        .mount("/fortune/template", openapi_get_routes![
            bill_book_template_controller::list,
        ])
        .mount("/fortune/bill", openapi_get_routes![
            bill_controller::add,
        ])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
    
}

