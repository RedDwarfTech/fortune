use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl,TextExpressionMethods};
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::map_entity;

use rust_wheel::config::db::config;

use crate::model::diesel::fortune::fortune_models::BillBookTemplate;
use crate::model::response::template::template_response::TemplateResponse;

pub fn get_template_list(filter_type: i32, filter_name: Option<String>) -> Vec<TemplateResponse> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_template as bill_book_template_table;
    let mut query = bill_book_template_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(bill_book_template_table::name.like(format!("{}{}{}", "%", some_filter_name.as_str(), "%")));
    }
    query = query.filter(bill_book_template_table::template_type.eq(filter_type).and(bill_book_template_table::online.eq(1)));
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let templates = query
        .load::<BillBookTemplate>(&connection)
        .expect("error get user contents");
    let template_results = map_entity(templates);
    return template_results;
}

pub fn get_template_detail(template_id: i32) -> Result<TemplateResponse,String> {
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
    let template_response = TemplateResponse::from(&query.get(0).unwrap().to_owned());
    return Ok(template_response)
}
