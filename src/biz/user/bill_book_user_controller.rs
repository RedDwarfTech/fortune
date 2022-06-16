use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::request::bill::bill_add_request::BillAddRequest;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use crate::model::request::user::user_action_request::UserActionRequest;
use crate::service::user::bill_book_user_service::query_user_actions;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page]
}

/// # 查询记账人列表
///
/// 查询记账人列表。
#[openapi(tag = "账本用户")]
#[get("/v1/page?<query..>")]
pub fn page(query: UserActionRequest) -> content::RawJson<String> {
    let query_results = query_user_actions(&query);
    return box_rest_response(query_results);
}

