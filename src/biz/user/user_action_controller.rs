use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::request::bill::bill_request::BillRequest;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page]
}

/// # 查询操作日志
///
/// 分页查询用户操作日志
#[openapi(tag = "操作日志")]
#[post("/v1/page", data = "<request>")]
pub fn page(request: Json<BillRequest>) -> content::RawJson<String> {
    return box_rest_response("contents");
}

