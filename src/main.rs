#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::{Build, Rocket};

mod common;
mod biz;
mod model;
mod service;

use rocket_okapi::{mount_endpoints_and_merged_docs, rapidoc::*, swagger_ui::*};
use rocket_okapi::settings::UrlObject;

use common::health_controller;
use biz::contents::contents_controller;
use biz::template::bill_book_template_controller;
use biz::bill::bill_controller;
use biz::bill::bill_book_controller;


#[rocket::main]
async fn main() {
    // https://github.com/GREsau/okapi/issues/99
    let launch_result = create_server().launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

pub fn create_server() -> Rocket<Build> {
    let mut building_rocket = rocket::build()
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("My special documentation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        );

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    //let custom_route_spec = (vec![], custom_openapi_spec());
    mount_endpoints_and_merged_docs! {
        building_rocket, "/v1".to_owned(), openapi_settings,
        //"/external" => custom_route_spec,
        "/fortune/actuator" => health_controller::get_routes_and_docs(&openapi_settings),
        "/fortune/template" => bill_book_template_controller::get_routes_and_docs(&openapi_settings),
        "/fortune/contents" => contents_controller::get_routes_and_docs(&openapi_settings),
        "/fortune/bill" => bill_controller::get_routes_and_docs(&openapi_settings),
        "/fortune/bill-book" => bill_book_controller::get_routes_and_docs(&openapi_settings),
        //"/message" => message::get_routes_and_docs(&openapi_settings),
    };

    building_rocket
}
