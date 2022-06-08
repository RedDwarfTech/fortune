use okapi::openapi3::OpenApi;
use rocket::response::content;
use rust_wheel::common::util::model_convert::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::diesel::fortune::fortune_models::FortuneContent;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: tree, fetch_available_contents]
}

#[openapi(tag = "账本-Legacy")]
#[get("/v1/tree?<contents_type>")]
pub fn tree(contents_type: i32, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    return box_rest_response("contents");
}

#[openapi(tag = "账本-Legacy")]
#[get("/v1/available?<contents_type>")]
pub fn fetch_available_contents(contents_type: i32, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    return box_rest_response("contents");
}
