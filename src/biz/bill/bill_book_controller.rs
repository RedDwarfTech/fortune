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
use crate::service::bill::bill_book_service::{add_bill_book, get_template_list};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list, add]
}

#[openapi(tag = "账本")]
#[get("/v1/list")]
pub fn list() -> content::RawJson<String> {
    let contents = get_template_list();
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
        Ok(_) => {
            box_rest_response(bill_book.unwrap())
        },
        Err(_) => {
            box_rest_response(bill_book.unwrap_err())
        }
    }
}
