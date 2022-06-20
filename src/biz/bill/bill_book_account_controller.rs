use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::util::model_convert::box_rest_response;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::bill::account::bill_account_request::BillAccountRequest;
use crate::model::request::bill::book::bill_book_request::BillBookRequest;
use crate::service::bill::bill_book_account_service::{add_bill_book, get_bill_book_by_id, get_bill_book_account_list};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list, add, detail]
}

/// # 查询当前用户账本账户列表
///
/// 按照账户分类查看
#[openapi(tag = "账本账户")]
#[get("/v1/list?<query..>")]
pub fn list(query: BillAccountRequest ) -> content::RawJson<String> {
    let contents = get_bill_book_account_list(&query);
    return box_rest_response(contents);
}

/// # 查询账本账户详情
///
/// 查询账本详情
#[openapi(tag = "账本账户")]
#[get("/v1/detail/<id>")]
pub fn detail(id: i64) -> content::RawJson<String> {
    let contents = get_bill_book_by_id(&id);
    return box_rest_response(contents);
}

/// # 新增账本账户
///
/// 新增不同类型的账本
#[openapi(tag = "账本账户")]
#[post("/v1/add", data = "<request>")]
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

