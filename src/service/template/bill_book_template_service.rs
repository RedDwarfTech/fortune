use diesel::RunQueryDsl;
use rust_wheel::config::db::config;

use crate::model::diesel::fortune::fortune_models::BillBookTemplate;

pub fn get_template_list() -> Vec<BillBookTemplate> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_template::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let templates = bill_book_template
        .load::<BillBookTemplate>(&connection)
        .expect("error get user contents");
    return templates;
}

