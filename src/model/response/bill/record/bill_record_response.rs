use serde::Serialize;
use crate::model::diesel::fortune::fortune_models::{ BillRecord };
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, Serialize, JsonSchema, Default)]
pub struct BillRecordResponse{
    pub id: i64,
    /// 金额
    pub amount: i64,
    /// 分类
    pub bill_book_contents_id: i64,
    /// 账户ID
    pub account_id: i64,
    /// 备注
    pub remark: Option<String>,
    /// 时间
    pub created_time: i64
}

impl From<&BillRecord> for BillRecordResponse {
    fn from(p: &BillRecord) -> Self {
        Self {
            id: p.id,
            amount: p.amount,
            bill_book_contents_id: p.bill_book_contents_id,
            account_id: p.account_id,
            remark: p.remark.to_owned(),
            created_time: p.created_time
        }
    }
}

