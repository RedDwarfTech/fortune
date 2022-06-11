use okapi::openapi3::OpenApi;
/**
To make the clion show unused imports

go to Settings > Editor > Inspections > Rust > Lints > Unused Import, enable it, and now CTRL+ALT+O will remove unused imports!

https://stackoverflow.com/questions/61077692/how-can-i-fix-unused-imports-in-rust-automatically
**/

use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::util::model_convert::box_rest_response;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::bill::bill_book_request::BillBookRequest;
use crate::service::bill::bill_book_service::{add_bill_book, get_bill_book_by_id, get_bill_book_list};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list, add, detail]
}

/// # 查询当前用户账本列表
///
/// 查询当前用户所属的账本，前期查询当前用户创建的账本，后期可查询用户创建的和参与的账本
#[openapi(tag = "账本")]
#[get("/v1/list?<name>")]
pub fn list(name: Option<String>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let contents = get_bill_book_list(name,&login_user_info);
    return box_rest_response(contents);
}

/// # 查询账本详情
///
/// 查询账本详情
#[openapi(tag = "账本")]
#[get("/v1/detail/<id>")]
pub fn detail(id: i64) -> content::RawJson<String> {
    let contents = get_bill_book_by_id(&id);
    return box_rest_response(contents);
}

/// # 新增账本
///
/// 新增不同类型的账本
#[openapi(tag = "账本")]
#[put("/v1/add", data = "<request>")]
pub fn add(request: Json<BillBookRequest>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let bill_book = add_bill_book(&request, &login_user_info);
    return match bill_book {
        Ok(v) => {
            box_rest_response(v)
        },
        Err(e) => {
            box_rest_response(e.to_string())
        }
    }
}

