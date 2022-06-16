use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, FromForm, JsonSchema)]
#[allow(non_snake_case)]
pub struct BillPageRequest {
    /// 页数
    pub pageNum: i64,
    /// 每页大小
    pub pageSize: i64,
    /// 账本ID
    pub bill_book_id: i64
}