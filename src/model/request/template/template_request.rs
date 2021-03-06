use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, FromForm, JsonSchema)]
#[allow(non_snake_case)]
pub struct TemplateRequest {
    /// 账本模版类型(1：家庭2：个人3：生意4：爱好)
    pub template_type: i32,
    /// 账本模版名称
    pub name: Option<String>
}
