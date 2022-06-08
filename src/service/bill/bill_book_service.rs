use diesel::RunQueryDsl;
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::fortune::fortune_custom_models::{BillBookAdd, BillRecordAdd};

use crate::model::diesel::fortune::fortune_models::{BillBook, BillBookTemplate};
use crate::model::request::bill::bill_book_request::BillBookRequest;

pub fn get_template_list() -> Vec<BillBookTemplate> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_template::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let templates = bill_book_template
        .load::<BillBookTemplate>(&connection)
        .expect("error get user contents");
    return templates;
}

pub fn add_bill_book(request: &Json<BillBookRequest>, login_user_info: &LoginUserInfo) {
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let bill_book_record = BillBookAdd{
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        deleted: 0,
        creator: login_user_info.userId,
        remark: None,
        bill_book_template_id: request.billBookTemplateId,
        contents: None
    };
    diesel::insert_into(crate::model::diesel::fortune::fortune_schema::bill_book::table)
        .values(&bill_book_record)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
}