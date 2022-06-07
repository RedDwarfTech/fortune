use serde::Serialize;
use crate::model::diesel::fortune::fortune_models::FortuneContent;

#[derive(Debug, Serialize, Default)]
pub struct EditContentResponse {
    pub own: Vec<FortuneContent>,
    pub available: Vec<FortuneContent>
}
