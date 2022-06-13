use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl,TextExpressionMethods};
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::{box_error_rest_response, box_error_type_rest_response, box_type_rest_response, map_entity};

use rust_wheel::config::db::config;
use rust_wheel::model::response::api_response::ApiResponse;

use crate::model::diesel::fortune::fortune_models::BillBookTemplate;
use crate::model::response::template::template_response::TemplateResponse;
use crate::utils::database::get_connection;

pub fn get_template_list(filter_type: i32, filter_name: Option<String>) -> Vec<TemplateResponse> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_template as bill_book_template_table;
    let mut query = bill_book_template_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(bill_book_template_table::name.like(format!("{}{}{}", "%", some_filter_name.as_str(), "%")));
    }
    query = query.filter(bill_book_template_table::template_type.eq(filter_type)
        .and(bill_book_template_table::online.eq(1)));
    let connection = get_connection();
    let templates = query
        .load::<BillBookTemplate>(&connection)
        .expect("error get bill book template");
    let template_results = map_entity(templates);
    return template_results;
}

pub fn get_template_detail(template_id: i64) -> Result<Json<ApiResponse<TemplateResponse>>,Json<ApiResponse<String>>> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_template as bill_book_template_table;
    let connection = get_connection();
    let query = bill_book_template_table::table
        .filter(bill_book_template_table::id.eq(template_id))
        .limit(1)
        .load::<BillBookTemplate>(&connection)
        .expect("query bill book template failed");
    if query.is_empty() {
        let err_resp:ApiResponse<String> = box_error_type_rest_response("".parse().unwrap(), "TEMPLATE_NOT_FOUND".parse().unwrap(), "not found template".parse().unwrap());
        return Err(Json::from(err_resp));
    }
    let template_response = TemplateResponse::from(&query.get(0).unwrap().to_owned());
    return Ok(Json::from(box_type_rest_response(template_response)))
}
