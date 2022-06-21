use serde::Serialize;
use crate::model::diesel::fortune::fortune_models::{ BillBook };
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, Serialize, JsonSchema, Default)]
pub struct BillBookResponse {
    pub id: i64,
    /// 账户名称
    pub name: String,
    /// 图标地址
    pub icon_url: String,
    /// 
    pub bill_book_template_id: i64,
    pub created_time: i64
}

impl From<&BillBook> for BillBookResponse {
    fn from(p: &BillBook) -> Self {
        Self {
            id: p.id,
            name: p.name.to_string(),
            icon_url: "".to_string(),
            bill_book_template_id: p.bill_book_template_id,
            created_time: p.created_time
        }
    }
}

