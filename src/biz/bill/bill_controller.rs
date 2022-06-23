use okapi::openapi3::OpenApi;
/**
To make the clion show unused imports

go to Settings > Editor > Inspections > Rust > Lints > Unused Import, enable it, and now CTRL+ALT+O will remove unused imports!

https://stackoverflow.com/questions/61077692/how-can-i-fix-unused-imports-in-rust-automatically
**/

use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::{box_rest_response, box_type_rest_response};
use rust_wheel::model::response::api_response::ApiResponse;

use crate::model::request::bill::bill_del_request::BillDelRequest;
use crate::model::request::bill::bill_edit_request::BillEditRequest;
use crate::model::response::bill::record::bill_record_response::BillRecordResponse;
use crate::service::bill::bill_service::{add_bill, archive_bill_book, query_bill_record_detail, query_bill_records, query_recoverable_records, recover_bill_record, del_bill_record, edit_bill_record};
use crate::model::request::bill::bill_add_request::BillAddRequest;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::bill::bill_detail_request::BillDetailRequest;
use crate::model::request::bill::bill_page_request::BillPageRequest;
use crate::model::request::bill::book::bill_book_archive_request::BillBookArchiveRequest;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: add, page, detail, archive, recoverable, recover, del, edit]
}

/// # 新增记账流水
///
/// 新增记账流水
#[openapi(tag = "账单流水")]
#[post("/v1/add", data = "<request>")]
pub fn add(request: Json<BillAddRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<Option<BillRecordResponse>>> {
    let contents = add_bill(request, &login_user_info);
    return match contents {
        Ok(v) => {
            let response = BillRecordResponse::from(&v);
            let boxed_response = box_type_rest_response(Some(response));
            Json::from(boxed_response)
        },
        Err(e) => {
            Json::from(box_type_rest_response(None))
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
pub fn detail(query: BillDetailRequest) -> Json<ApiResponse<BillRecordResponse>> {
    let contents = query_bill_record_detail(&query);
    let response = BillRecordResponse::from(&contents);
    let boxed_response = box_type_rest_response(response);
    return Json::from(boxed_response);
}

/// # 删除记账流水
///
/// 删除记账流水
#[openapi(tag = "账单流水")]
#[delete("/v1/delete?<query..>")]
pub fn del(query: BillDelRequest, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let del_result = del_bill_record(&query, &login_user_info);
    return match del_result {
        Ok(v) => {
            box_rest_response(v)
        },
        Err(e) => {
            box_rest_response(e.to_string())
        }
    }
}

/// # 编辑记账流水
///
/// 编辑记账流水
#[openapi(tag = "账单流水")]
#[patch("/v1/edit", data = "<request>")]
pub fn edit(request: Json<BillEditRequest>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    edit_bill_record(&request, &login_user_info);
    return box_rest_response("ok");
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
    let contents = recover_bill_record(&query);
    return box_rest_response(contents);
}