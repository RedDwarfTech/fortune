use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, FromForm, JsonSchema)]
#[allow(non_snake_case)]
pub struct UserActionRequest {
    /// 账本ID
    pub bill_book_id: i64,
    /// 每页数量
    pub limit: i64,
    /// 偏移量
    pub offset: i64
}