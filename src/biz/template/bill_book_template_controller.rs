use okapi::openapi3::OpenApi;
/**
To make the clion show unused imports

go to Settings > Editor > Inspections > Rust > Lints > Unused Import, enable it, and now CTRL+ALT+O will remove unused imports!

https://stackoverflow.com/questions/61077692/how-can-i-fix-unused-imports-in-rust-automatically
**/

use rocket::response::content;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::service::template::bill_book_template_service::{get_template_detail, get_template_list};
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list, detail]
}

/// # 查询账本模版列表
///
/// 返回不同类型的账本模版列表
#[openapi(tag = "账本模版")]
#[get("/v1/list?<template_type>&<name>")]
pub fn list(template_type: i32, name: Option<String>) -> content::RawJson<String> {
    let contents = get_template_list(template_type, name);
    return box_rest_response(contents);
}

/// # 查询账本模版详情
///
/// 根据账本模版ID查询账本详情
#[openapi(tag = "账本模版")]
#[get("/v1/detail/<id>")]
pub fn detail(id: i32) -> content::RawJson<String> {
    let contents = get_template_detail(id);
    return match contents {
        Ok(_) => {
            return box_rest_response(contents.unwrap());
            box_rest_response(contents.unwrap())
        },
        Err(_) => {
            return box_rest_response(contents.unwrap_err());
        }
    }
}