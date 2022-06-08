use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rust_wheel::config::db::config;

use crate::model::diesel::fortune::fortune_models::BillBookTemplate;

pub fn get_template_list(filter_type: i32) -> Vec<BillBookTemplate> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_template::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let predicate = template_type.eq(filter_type);
    let templates = bill_book_template
        .filter(predicate)
        .load::<BillBookTemplate>(&connection)
        .expect("error get user contents");
    return templates;
}

