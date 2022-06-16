use okapi::openapi3::OpenApi;
/**
To make the clion show unused imports

go to Settings > Editor > Inspections > Rust > Lints > Unused Import, enable it, and now CTRL+ALT+O will remove unused imports!

https://stackoverflow.com/questions/61077692/how-can-i-fix-unused-imports-in-rust-automatically
**/

use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::service::bill::bill_service::{add_bill, archive_bill_book, query_bill_record_detail, query_bill_records, query_recoverable_records};
use crate::model::request::bill::bill_add_request::BillAddRequest;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::bill::bill_book_archive_request::BillBookArchiveRequest;
use crate::model::request::bill::bill_detail_request::BillDetailRequest;
use crate::model::request::bill::bill_page_request::BillPageRequest;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add, page, detail, archive, recoverable, recover]
}

/// # 新增记账流水
///
/// 新增记账流水
#[openapi(tag = "账单流水")]
#[post("/v1/add", data = "<request>")]
pub fn add(request: Json<BillAddRequest>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let contents = add_bill(request, &login_user_info);
    return match contents {
        Ok(v) => {
            box_rest_response(v)
        },
        Err(e) => {
            box_rest_response(e.to_string())
        }
    }
}

/// # 查询记账流水
///
/// 分页查询记账流水
#[openapi(tag = "账单流水")]
#[get("/v1/page?<query..>")]
pub fn page(query: BillPageRequest) -> content::RawJson<String> {
    let contents = query_bill_records(&query);
    return box_rest_response(contents);
}


/// # 查询记账流水详情
///
/// 查询记账流水详情
#[openapi(tag = "账单流水")]
#[get("/v1/detail?<query..>")]
pub fn detail(query: BillDetailRequest) -> content::RawJson<String> {
    let contents = query_bill_record_detail(&query);
    return box_rest_response(contents);
}

/// # 封账(VIP)
///
/// 封存部分流水防止修改
#[openapi(tag = "账单流水")]
#[patch("/v1/archive?<query..>")]
pub fn archive(query: BillBookArchiveRequest) -> content::RawJson<String> {
    archive_bill_book(&query);
    return box_rest_response("ok");
}

/// # 查询可恢复流水
///
/// 查询部分可恢复流水
#[openapi(tag = "账单流水")]
#[get("/v1/recover/list?<query..>")]
pub fn recoverable(query: BillPageRequest) -> content::RawJson<String> {
    let contents = query_recoverable_records(&query);
    return box_rest_response(contents);
}

/// # 恢复流水(VIP)
///
/// 恢复流水
#[openapi(tag = "账单流水")]
#[patch("/v1/recover?<query..>")]
pub fn recover(query: BillPageRequest) -> content::RawJson<String> {
    let contents = query_bill_records(&query);
    return box_rest_response(contents);
}