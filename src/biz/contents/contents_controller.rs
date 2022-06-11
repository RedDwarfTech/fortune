use okapi::openapi3::OpenApi;
use rocket::form::Form;
use rocket::response::content;
use rust_wheel::common::util::model_convert::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use crate::model::request::contents::contents_request::ContentsRequest;
use crate::service::contents::contents_service::content_tree_query;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: tree, add_contents, del_contents, edit_contents]
}

/// # 查询账本分类目录
///
/// 获取当前账本的分类目录，不同的账本模版分类目录不同。返回树形结构的数据。
#[openapi(tag = "账本分类目录")]
#[get("/v1/tree?<query..>")]
pub fn tree(query: ContentsRequest, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let contents =  content_tree_query(query.contents_type, query.bill_book_id);
    return box_rest_response(contents);
}

/// # 新增账本分类
///
/// 新增某一个特定账本的分类，每一个账本都有单独的分类，注意新增A账本的分类不会影响B账本的分类
#[openapi(tag = "账本分类目录")]
#[post("/v1/contents?<contents_type>")]
pub fn add_contents(contents_type: i32, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    return box_rest_response("contents");
}

/// # 删除账本分类
///
/// 删除某一个特定账本的分类，每一个账本都有单独的分类，注意删除A账本的分类不会影响B账本的分类
#[openapi(tag = "账本分类目录")]
#[delete("/v1/contents?<contents_type>")]
pub fn del_contents(contents_type: i32, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    return box_rest_response("contents");
}

/// # 编辑账本分类
///
/// 编辑某一个特定账本的分类，每一个账本都有单独的分类，注意编辑A账本的分类不会影响B账本的分类
// why use patch?
// https://coolshell.cn/articles/22173.html
// PUT 和 PATCH 都是更新业务资源信息，如果资源对象不存在则可以新建一个，但他们两者的区别是，
// PUT 用于更新一个业务对象的所有完整信息，就像是我们通过表单提交所有的数据，
// 而 PATCH 则对更为API化的数据更新操作，只需要更需要更新的字段（参看 RFC 5789 ）。
#[openapi(tag = "账本分类目录")]
#[patch("/v1/contents?<contents_type>")]
pub fn edit_contents(contents_type: i32, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    return box_rest_response("contents");
}