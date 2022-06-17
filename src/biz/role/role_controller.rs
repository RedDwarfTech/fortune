use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::request::role::add_role_request::AddRoleRequest;
use crate::model::request::role::bill_book_role_request::BillBookRoleRequest;
use crate::model::request::role::role_list_request::RoleListRequest;
use crate::service::role::role_service::{add_bill_book_role, query_bill_book_roles, query_bill_book_roles_detail};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list, add, edit, del, detail]
}

/// # 查询账本角色列表
///
/// 查询账本角色列表，账本之间角色独立。
#[openapi(tag = "角色")]
#[get("/v1/list?<query..>")]
pub fn list(query: RoleListRequest, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let roles = query_bill_book_roles(&query,&login_user_info);
    return box_rest_response(roles);
}

/// # 查询账本角色详情
///
/// 查询账本角色详情，账本之间角色独立。
#[openapi(tag = "角色")]
#[get("/v1/detail?<query..>")]
pub fn detail(query: BillBookRoleRequest, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let roles = query_bill_book_roles_detail(&query,&login_user_info);
    return box_rest_response(roles);
}

/// # 添加账本自定义角色
///
/// 为当前账本新增自定义角色。
#[openapi(tag = "角色")]
#[post("/v1/add", data = "<request>")]
pub fn add(request: Json<AddRoleRequest>, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let roles = add_bill_book_role(&request,&login_user_info);
    return match roles {
        Ok(v) => {
            box_rest_response(v)
        },
        Err(e) => {
            box_rest_response(e.to_string())
        }
    }
}

/// # 编辑账本自定义角色
///
/// 编辑当前账本自定义角色。
#[openapi(tag = "角色")]
#[patch("/v1/edit?<query..>")]
pub fn edit(query: RoleListRequest, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let roles = query_bill_book_roles(&query,&login_user_info);
    return box_rest_response(roles);
}

/// # 删除账本自定义角色
///
/// 删除当前账本自定义角色。
#[openapi(tag = "角色")]
#[delete("/v1/del?<query..>")]
pub fn del(query: RoleListRequest, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let roles = query_bill_book_roles(&query,&login_user_info);
    return box_rest_response(roles);
}