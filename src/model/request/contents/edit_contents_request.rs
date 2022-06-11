use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, FromForm, JsonSchema)]
#[allow(non_snake_case)]
pub struct EditContentsRequest {
    /// 名称
    pub name: String,
    /// 账本分类ID
    pub id: i64,
    /// 账本ID
    pub bill_book_id: i64
}
