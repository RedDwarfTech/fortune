use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct BillRequest {
    /// 账本ID
    pub billBookId: i64,
    pub contentsId: i64,
    /// 金额
    pub amount: i64
}