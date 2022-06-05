use diesel::{BoolExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use diesel::dsl::any;
use rocket::serde::json::serde_json;
use rust_wheel::common::util::convert_to_tree::convert_to_tree;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::fortune::fortune_models::{FortuneContent, FortuneUserContent};
use crate::diesel::ExpressionMethods;
use crate::model::response::contents::fortune_contents_response::FortuneContentResponse;

///
/// 每个用户看到的菜单都不一样
/// 所以先查询当前用户拥有的菜单ID集合
/// 再组装成当前用户的菜单树
///
/// 针对系统菜单的增减
/// 后期用户可以在界面上进行区分
/// 会显示2个区域，一个是已经有的记账类型，一个是目前可以加入的记账类型
///
pub fn content_tree_query<T>(filter_content_type: i32, login_user_info: LoginUserInfo) -> Vec<FortuneContentResponse> {
    use crate::model::diesel::fortune::fortune_schema::fortune_contents::dsl::*;
    use crate::model::diesel::fortune::fortune_schema::fortune_user_contents::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let user_contents = fortune_user_contents.filter(user_id.eq(login_user_info.userId))
        .load::<FortuneUserContent>(&connection)
        .expect("error get user contents");
    if user_contents.is_empty() {
        return Vec::new();
    }
    let contents_ids: Vec<i32> = user_contents.iter()
        .map(|item| item.contents_id)
        .collect();
    let predicate = contents_type.eq(filter_content_type)
        .and(crate::model::diesel::fortune::fortune_schema::fortune_contents::dsl::id.eq(any(contents_ids)));
    let contents = fortune_contents.filter(&predicate)
        .load::<FortuneContent>(&connection)
        .expect("Error fortune contents resource");
    return convert_to_tree_impl(&contents);
}

///
/// 获取可以加入用户账单分类的类型
///
pub fn get_available_content(contents_ids: Vec<i32>, connection: &PgConnection, filter_content_type: &i32) -> Vec<FortuneContent>{
    use crate::model::diesel::fortune::fortune_schema::fortune_contents::dsl::*;
    let predicate = contents_type.eq(filter_content_type)
        .and(crate::model::diesel::fortune::fortune_schema::fortune_contents::dsl::id.ne(any(contents_ids)))
        .and(contents_source.eq(1));
    let contents = fortune_contents.filter(&predicate)
        .load::<FortuneContent>(connection)
        .expect("Error fortune contents resource");
    return contents;
}

pub fn convert_to_tree_impl(contents: &Vec<FortuneContent>) -> Vec<FortuneContentResponse> {
    let root_element: Vec<_> = contents.iter()
        .filter(|content| content.parent_id == 0)
        .collect();
    let sub_element: Vec<_> = contents.iter()
        .filter(|content| content.parent_id != 0)
        .collect();
    let result = convert_to_tree(&root_element, &sub_element);
    return result;
}

