use rocket::response::content;
use rocket::serde::json::Json;
use crate::model::request::contents::contents_request::ContentsRequest;
use rust_wheel::common::util::model_convert::box_rest_response;
use crate::model::diesel::fortune::fortune_models::FortuneContent;
use crate::service::contents::contents_service::content_tree_query;

#[get("/v1/tree?<contents_type>")]
pub fn page(contents_type: i32) -> content::RawJson<String> {
    let contents = content_tree_query::<Vec<FortuneContent>>(contents_type);
    return box_rest_response("users");
}


