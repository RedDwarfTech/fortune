use okapi::openapi3::OpenApi;
/**
To make the clion show unused imports

go to Settings > Editor > Inspections > Rust > Lints > Unused Import, enable it, and now CTRL+ALT+O will remove unused imports!

https://stackoverflow.com/questions/61077692/how-can-i-fix-unused-imports-in-rust-automatically
**/

use rocket::response::content;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::service::template::bill_book_template_service::get_template_list;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list]
}

#[openapi(tag = "账本模版")]
#[get("/v1/list?<template_type>")]
pub fn list(template_type: i32) -> content::RawJson<String> {
    let contents = get_template_list(template_type);
    return box_rest_response(contents);
}

