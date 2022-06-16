use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct BillEditRequest {
    /// 流水ID
    pub id: i64,
    /// 账本ID
    pub bill_book_id: i64,
    /// 账户类型ID
    pub account_id: i32,
    /// 流水分类ID
    pub bill_book_contents_id: i64,
    /// 金额
    pub amount: i64,
    /// 备注
    pub remark: Option<String>
}