use diesel::{QueryDsl, RunQueryDsl};
use rocket::serde::json::serde_json;
use rust_wheel::config::db::config;
use crate::model::diesel::fortune::fortune_models::FortuneContent;
use crate::diesel::ExpressionMethods;
use crate::model::response::contents::fortune_contents_response::FortuneContentResponse;
use crate::service::contents::convert_to_tree::convert_to_tree;

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

