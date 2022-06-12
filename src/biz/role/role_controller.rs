use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::request::bill::bill_add_request::BillAddRequest;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::service::role::role_service::query_roles;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list]
}

/// # 查询角色列表
///
/// 查询角色列表
#[openapi(tag = "角色")]
#[get("/v1/list")]
pub fn list(login_user_info: LoginUserInfo) -> content::RawJson<String> {
    query_roles(&login_user_info);
    return box_rest_response("contents");
}

