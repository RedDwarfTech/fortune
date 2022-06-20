#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::{Build, Rocket};

mod common;
mod biz;
mod model;
mod service;
mod utils;
mod test;

use rocket_okapi::{mount_endpoints_and_merged_docs, OpenApiError, rapidoc::*, swagger_ui::*};
use rocket_okapi::settings::UrlObject;

use common::health_controller;
use biz::contents::contents_controller;
use biz::template::bill_book_template_controller;
use biz::bill::bill_controller;
use biz::bill::bill_book_controller;
use biz::bill::bill_book_account_controller;
use biz::user::user_action_controller;
use biz::user::bill_book_user_controller;
use biz::role::role_controller;
use biz::permission::permission_controller;

pub type Result<T> = std::result::Result<T, OpenApiError>;


#[rocket::main]
async fn main() {
    // the performance about rocket and Actix
    // https://www.youtube.com/watch?v=GAxxn_oGA0Y
    // https://stackoverflow.com/questions/72540558/what-is-the-difference-about-rocket-launch-and-main-entrypoint
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
                url: "../fortune/openapi.json".to_owned(),
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
        building_rocket, "/fortune".to_owned(), openapi_settings,
        "/actuator" => health_controller::get_routes_and_docs(&openapi_settings),
        "/template" => bill_book_template_controller::get_routes_and_docs(&openapi_settings),
        "/contents" => contents_controller::get_routes_and_docs(&openapi_settings),
        "/bill" => bill_controller::get_routes_and_docs(&openapi_settings),
        "/bill-book" => bill_book_controller::get_routes_and_docs(&openapi_settings),
        "/bill-book-account" => bill_book_account_controller::get_routes_and_docs(&openapi_settings),
        "/user-action" => user_action_controller::get_routes_and_docs(&openapi_settings),
        "/role" => role_controller::get_routes_and_docs(&openapi_settings),
        "/permission" => permission_controller::get_routes_and_docs(&openapi_settings),
        "/bill-book-user" => bill_book_user_controller::get_routes_and_docs(&openapi_settings),
    };

    building_rocket
}
