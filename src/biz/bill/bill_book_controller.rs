use okapi::openapi3::OpenApi;
/**
To make the clion show unused imports

go to Settings > Editor > Inspections > Rust > Lints > Unused Import, enable it, and now CTRL+ALT+O will remove unused imports!

https://stackoverflow.com/questions/61077692/how-can-i-fix-unused-imports-in-rust-automatically
**/
use rocket::serde::json::Json;
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::util::model_convert::{box_type_rest_response, map_entity};
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rust_wheel::model::response::api_response::ApiResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::bill::book::bill_book_edit_request::BillBookEditRequest;
use crate::model::request::bill::book::bill_book_request::BillBookRequest;
use crate::model::response::bill::book::bill_book_response::BillBookResponse;
use crate::service::bill::bill_book_service::{add_bill_book, get_bill_book_by_id, get_bill_book_list, edit_bill_book};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list, add, detail, edit]
}

/// # 查询当前用户账本列表
///
/// 查询当前用户所属的账本，前期查询当前用户创建的账本，后期可查询用户创建的和参与的账本
#[openapi(tag = "账本")]
#[get("/v1/list?<name>")]
pub fn list(name: Option<String>, login_user_info: LoginUserInfo) -> Json<ApiResponse<Vec<BillBookResponse>>> {
    let contents = get_bill_book_list(name,&login_user_info);
    let map_result = map_entity(contents);
    let boxed_result = box_type_rest_response(map_result);
    return Json::from(boxed_result);
}

/// # 查询账本详情
///
/// 查询账本详情
#[openapi(tag = "账本")]
#[get("/v1/detail/<id>")]
pub fn detail(id: i64) -> Json<ApiResponse<BillBookResponse>> {
    let contents = get_bill_book_by_id(&id);
    let bill_book_res  = BillBookResponse::from(&contents);
    let boxed_response = box_type_rest_response(bill_book_res);
    return Json::from(boxed_response);
}

/// # 编辑账本
///
/// 编辑账本信息
#[openapi(tag = "账本")]
#[patch("/v1/edit", data = "<request>")]
pub fn edit(request: Json<BillBookEditRequest>) -> Json<ApiResponse<BillBookResponse>> {
    let contents = edit_bill_book(&request);
    let bill_book_response = BillBookResponse::from(&contents);
    let boxed_response = box_type_rest_response(bill_book_response);
    return Json::from(boxed_response);
}

/// # 新增账本
///
/// 新增不同类型的账本
#[openapi(tag = "账本")]
#[post("/v1/add", data = "<request>")]
pub fn add(request: Json<BillBookRequest>, login_user_info: LoginUserInfo) -> Json<ApiResponse<Option<BillBookResponse>>> {
    let bill_book = add_bill_book(&request, &login_user_info);
    return match bill_book {
        Ok(v) => {
            let bill_book_response = BillBookResponse::from(&v);
            let bill_book_boxed = box_type_rest_response(Some(bill_book_response));
            Json::from(bill_book_boxed)
        },
        Err(_e) => {
            Json::from(box_type_rest_response(None))
        }
    }
}