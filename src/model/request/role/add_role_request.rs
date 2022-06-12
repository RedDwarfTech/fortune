use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[allow(non_snake_case)]
pub struct AddRoleRequest {
    /// 账本ID
    pub bill_book_id: i64,
    /// 角色名称
    pub name: String,
    /// 备注
    pub remark: Option<String>
}
