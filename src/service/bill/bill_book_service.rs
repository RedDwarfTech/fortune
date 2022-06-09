use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::query_dsl::filter_dsl::FilterDsl;
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::fortune::fortune_custom_models::{BillBookAdd, BillRecordAdd};

use crate::model::diesel::fortune::fortune_models::{BillBook, BillBookTemplate, BillRecord};
use crate::model::request::bill::bill_book_request::BillBookRequest;

pub fn get_template_list() -> Vec<BillBookTemplate> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_template::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let templates = bill_book_template
        .load::<BillBookTemplate>(&connection)
        .expect("error get user contents");
    return templates;
}

fn get_template_list_by_id(template_id: i32) -> Vec<BillBookTemplate>{
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    use crate::model::diesel::fortune::fortune_schema::bill_book_template::dsl::*;
    let predicate = id.eq(template_id);
    let templates = bill_book_template
        .filter(predicate)
        .load::<BillBookTemplate>(&connection)
        .expect("error get user contents");
    return templates;
}

pub fn add_bill_book(request:&Json<BillBookRequest>, login_user_info: &LoginUserInfo) -> Result<BillBook,String> {
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let templates = get_template_list_by_id(request.billBookTemplateId);
    if templates.is_empty() {
        return Err("the template did not exists".parse().unwrap());
    }
    let bill_book_record = BillBookAdd{
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        deleted: 0,
        creator: login_user_info.userId,
        remark: None,
        bill_book_template_id: request.billBookTemplateId,
        contents: None
    };
    let inserted_record = diesel::insert_into(crate::model::diesel::fortune::fortune_schema::bill_book::table)
        .values(&bill_book_record)
        .on_conflict_do_nothing()
        .get_results::<BillBook>(&connection);
    let records = inserted_record.unwrap();
    // 使用to_owned()表示重新拷贝了一份数据，和重新构建一个String出来别无二致
    let r = records.get(0).unwrap().to_owned();
    return Ok(r);
}