use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rust_wheel::config::db::config;

use crate::model::diesel::fortune::fortune_models::BillBookTemplate;

pub fn get_template_list(filter_type: i32, filter_name: Option<String>) -> Vec<BillBookTemplate> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_template as bill_book_template_table;
    let mut query = bill_book_template_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(bill_book_template_table::name.eq(some_filter_name));
    }
    query = query.filter(bill_book_template_table::template_type.eq(filter_type));
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let templates = query
        .load::<BillBookTemplate>(&connection)
        .expect("error get user contents");
    return templates;
}

