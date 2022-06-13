use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::request::bill::bill_add_request::BillAddRequest;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use crate::model::request::user::user_action_request::UserActionRequest;
use crate::service::user::user_action_service::query_user_actions;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page]
}

/// # 查询操作日志
///
/// 分页查询用户操作日志
#[openapi(tag = "操作日志")]
#[get("/v1/page?<query..>")]
pub fn page(query: UserActionRequest) -> content::RawJson<String> {
    let query_results = query_user_actions(&query);
    return box_rest_response(query_results);
}

