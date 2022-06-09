use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};
use rust_wheel::config::db::config;

use crate::model::diesel::fortune::fortune_models::BillBookTemplate;

pub fn get_template_list(filter_type: i32, filter_name: Option<String>) -> Vec<BillBookTemplate> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_template as bill_book_template_table;
    let mut query = bill_book_template_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(bill_book_template_table::name.like(format!("{}{}{}", "%", some_filter_name.as_str(), "%")));
    }
    query = query.filter(bill_book_template_table::template_type.eq(filter_type));
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let templates = query
        .load::<BillBookTemplate>(&connection)
        .expect("error get user contents");
    return templates;
}

pub fn get_template_detail(template_id: i32) -> Result<BillBookTemplate,String> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_template as bill_book_template_table;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let query = bill_book_template_table::table
        .filter(bill_book_template_table::id.eq(template_id))
        .limit(1)
        .load::<BillBookTemplate>(&connection)
        .expect("query bill book template failed");
    if query.is_empty() {
        return Err("template is null".parse().unwrap());
    }
    return Ok(query.get(0).unwrap().to_owned())
}
