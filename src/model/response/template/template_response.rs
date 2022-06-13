use serde::Serialize;
use crate::model::diesel::fortune::fortune_models::BillBookTemplate;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, Serialize, JsonSchema, Default)]
pub struct TemplateResponse {
    pub id: i64,
    /// 账本模版名称
    pub name: String,
    pub tags: String,
    pub slogan: String,
    pub icon_url: String,
    pub user_count: i64,
    pub template_type: i32,
    pub remark: String
}

impl From<&BillBookTemplate> for TemplateResponse {
    fn from(p: &BillBookTemplate) -> Self {
        Self {
            id: p.id,
            name: p.name.to_string(),
            tags: p.tags.to_string(),
            slogan: p.slogan.to_string(),
            icon_url: p.icon_url.to_string(),
            user_count: p.user_count,
            template_type: p.template_type,
            remark: p.remark.to_string()
        }
    }
}

