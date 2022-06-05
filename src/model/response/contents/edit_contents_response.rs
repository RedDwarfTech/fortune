use rust_wheel::common::util::convert_to_tree::IntoTree;
use serde::Serialize;
use crate::model::diesel::fortune::fortune_models::FortuneContent;
use crate::model::response::contents::fortune_contents_response::FortuneContentResponse;

#[derive(Debug, Serialize, Default)]
pub struct EditContentResponse {
    pub own: Vec<FortuneContent>,
    pub available: Vec<FortuneContent>
}
