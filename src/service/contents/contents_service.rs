use diesel::{QueryDsl, RunQueryDsl};
use rust_wheel::config::db::config;
use crate::model::diesel::fortune::fortune_models::FortuneContent;
use crate::diesel::ExpressionMethods;

pub fn content_tree_query<T>(contents_type: i32) -> Vec<FortuneContent> {
    use crate::model::diesel::fortune::fortune_schema::fortune_contents::dsl::*;
    let connection = config::establish_connection();
    let predicate = crate::model::diesel::fortune::fortune_schema::fortune_contents::contents_type.eq(contents_type);
    let contents = fortune_contents.filter(&predicate)
        .load::<FortuneContent>(&connection)
        .expect("Error fortune contents resource");
    return contents;
}