use rocket::response::content;
use rocket::serde::json::Json;
use crate::model::request::contents::contents_request::ContentsRequest;
use rust_wheel::common::util::model_convert::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::fortune::fortune_models::FortuneContent;
use crate::service::contents::contents_service::content_tree_query;

#[get("/v1/tree?<contents_type>")]
pub fn tree(contents_type: i32, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let contents = content_tree_query::<Vec<FortuneContent>>(contents_type, login_user_info);
    return box_rest_response(contents);
}


