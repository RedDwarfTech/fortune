use okapi::openapi3::OpenApi;
use rocket::{Request, Response, response};
/**
To make the clion show unused imports

go to Settings > Editor > Inspections > Rust > Lints > Unused Import, enable it, and now CTRL+ALT+O will remove unused imports!

https://stackoverflow.com/questions/61077692/how-can-i-fix-unused-imports-in-rust-automatically
**/

use rocket::response::{content, Responder};
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::{box_rest_response, box_type_rest_response};

use crate::service::template::bill_book_template_service::{get_template_detail, get_template_list};
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::model::response::api_response::ApiResponse;
use crate::model::request::template::template_detail_request::TemplateDetailRequest;
use crate::model::request::template::template_request::TemplateRequest;
use crate::model::response::template::template_response::TemplateResponse;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list, detail]
}

/// # 查询账本模版列表
///
/// 返回不同类型的账本模版列表
#[openapi(tag = "账本模版")]
#[get("/v1/list?<query..>")]
pub fn list(query: TemplateRequest) -> Result<Json<ApiResponse<Vec<TemplateResponse>>>, String> {
    let contents = get_template_list(query.template_type, query.name);
    let boxed_response = box_type_rest_response(contents);
    return Ok(Json::from(boxed_response));
}

/// # 查询账本模版详情
///
/// 根据账本模版ID查询账本详情
#[openapi(tag = "账本模版")]
#[get("/v1/detail?<query..>")]
pub fn detail(query: TemplateDetailRequest) -> content::RawJson<String> {
    let contents = get_template_detail(query.id);
    return match contents {
        Ok(v) => {
            box_rest_response(v)
        },
        Err(e) => {
            box_rest_response(e)
        }
    }
}