use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::request::bill::bill_add_request::BillAddRequest;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page]
}

/// # 权限列表
///
/// 查询权限列表
#[openapi(tag = "权限")]
#[post("/v1/page", data = "<_request>")]
pub fn page(_request: Json<BillAddRequest>) -> content::RawJson<String> {
    return box_rest_response("contents");
}

