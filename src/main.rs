#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use okapi::openapi3::OpenApi;
use rocket::{Build, Rocket, routes};
use rocket::serde::json::Json;

mod common;
mod biz;
mod model;
mod service;

use rocket_okapi::{mount_endpoints_and_merged_docs, openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use serde::{Deserialize, Serialize};

use common::health_controller;
use biz::contents::contents_controller;
use biz::template::bill_book_template_controller;
use biz::bill::bill_controller;


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
        "/health" => health_controller::get_routes_and_docs(&openapi_settings),
        "/template" => bill_book_template_controller::get_routes_and_docs(&openapi_settings),
        //"/message" => message::get_routes_and_docs(&openapi_settings),
    };

    building_rocket
}

fn custom_openapi_spec() -> OpenApi {
    use indexmap::indexmap;
    use rocket_okapi::okapi::openapi3::*;
    use rocket_okapi::okapi::schemars::schema::*;
    OpenApi {
        openapi: OpenApi::default_version(),
        info: Info {
            title: "The best API ever".to_owned(),
            description: Some("This is the best API every, please use me!".to_owned()),
            terms_of_service: Some(
                "https://github.com/GREsau/okapi/blob/master/LICENSE".to_owned(),
            ),
            contact: Some(Contact {
                name: Some("okapi example".to_owned()),
                url: Some("https://github.com/GREsau/okapi".to_owned()),
                email: None,
                ..Default::default()
            }),
            license: Some(License {
                name: "MIT".to_owned(),
                url: Some("https://github.com/GREsau/okapi/blob/master/LICENSE".to_owned()),
                ..Default::default()
            }),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            ..Default::default()
        },
        servers: vec![
            Server {
                url: "http://127.0.0.1:8000/".to_owned(),
                description: Some("Localhost".to_owned()),
                ..Default::default()
            },
            Server {
                url: "https://example.com/".to_owned(),
                description: Some("Possible Remote".to_owned()),
                ..Default::default()
            },
        ],
        // Add paths that do not exist in Rocket (or add extra into to existing paths)
        paths: {
            indexmap! {
                "/home".to_owned() => PathItem{
                get: Some(
                    Operation {
                    tags: vec!["HomePage".to_owned()],
                    summary: Some("This is my homepage".to_owned()),
                    responses: Responses{
                        responses: indexmap!{
                        "200".to_owned() => RefOr::Object(
                            Response{
                            description: "Return the page, no error.".to_owned(),
                            content: indexmap!{
                                "text/html".to_owned() => MediaType{
                                schema: Some(SchemaObject{
                                    instance_type: Some(SingleOrVec::Single(Box::new(
                                        InstanceType::String
                                    ))),
                                    ..Default::default()
                                }),
                                ..Default::default()
                                }
                            },
                            ..Default::default()
                            }
                        )
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                    }
                ),
                ..Default::default()
                }
            }
        },
        ..Default::default()
    }
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

