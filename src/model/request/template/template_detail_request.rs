use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, FromForm, JsonSchema)]
#[allow(non_snake_case)]
pub struct TemplateDetailRequest {
    /// 账本模版ID
    pub id: i64,
}
