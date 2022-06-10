use okapi::openapi3::OpenApi;
use rocket::form::Form;
use rocket::response::content;
use rust_wheel::common::util::model_convert::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use crate::model::request::contents::contents_request::ContentsRequest;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: tree, fetch_available_contents]
}

/// # 查询账本分类目录
///
/// 获取当前账本的分类目录，不同的账本模版分类目录不同。
#[openapi(tag = "账本分类目录")]
#[get("/v1/tree?<query..>")]
pub fn tree(query: ContentsRequest, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    return box_rest_response("contents");
}

/// # xxxx
///
/// 新增记账流水
#[openapi(tag = "账本分类目录")]
#[get("/v1/available?<contents_type>")]
pub fn fetch_available_contents(contents_type: i32, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    return box_rest_response("contents");
}
