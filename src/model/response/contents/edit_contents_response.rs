use serde::Serialize;
use crate::model::diesel::fortune::fortune_custom_models::BillBookTemplateContents;

#[derive(Debug, Serialize, Default)]
pub struct EditContentResponse {
    pub own: Vec<BillBookTemplateContents>,
    pub available: Vec<BillBookTemplateContents>
}
