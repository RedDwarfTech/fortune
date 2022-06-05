use diesel::{QueryDsl, RunQueryDsl};
use rocket::serde::json::serde_json;
use rust_wheel::common::util::convert_to_tree::convert_to_tree;
use rust_wheel::config::db::config;
use crate::model::diesel::fortune::fortune_models::FortuneContent;
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
pub fn content_tree_query<T>(filter_content_type: i32) -> Vec<FortuneContentResponse> {
    use crate::model::diesel::fortune::fortune_schema::fortune_contents::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());





    let predicate = contents_type.eq(filter_content_type);
    let contents = fortune_contents.filter(&predicate)
        .load::<FortuneContent>(&connection)
        .expect("Error fortune contents resource");
    return convert_to_tree_impl(&contents);
}

pub fn convert_to_tree_impl(contents: &Vec<FortuneContent>) -> Vec<FortuneContentResponse>{
    let root_element:Vec<_> = contents.iter()
        .filter(|content|content.parent_id==0)
        .collect();
    let sub_element:Vec<_> =  contents.iter()
        .filter(|content|content.parent_id!=0)
        .collect();
    let result = convert_to_tree(&root_element, &sub_element);
    return result;
}

