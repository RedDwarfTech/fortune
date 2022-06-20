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
    /// 当前账户金额
    pub amount: i64,
    /// 账户类型
    pub account_type: i32
}

impl From<&BillBook> for BillBookResponse {
    fn from(p: &BillBook) -> Self {
        Self {
            id: p.id,
            name: p.name.to_string(),
            icon_url: "".to_string(),
            amount: todo!(),
            account_type: todo!(),
        }
    }
}

