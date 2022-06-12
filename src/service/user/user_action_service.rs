use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::{ diesel::fortune::fortune_custom_models::BillRecordAdd };
use crate::model::diesel::fortune::fortune_schema::bill_book::dsl::bill_book;
use crate::model::request::user::user_action_request::UserActionRequest;

pub fn query_user_actions(_request: &Json<UserActionRequest>) {
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    use crate::model::diesel::fortune::fortune_schema::user_action_log as action_log_table;
    let predicate = action_log_table::dsl::bill_book_id.eq(_request.bill_book_id);



    let bill_record_add = BillRecordAdd {
        created_time: todo!(),
        updated_time: todo!(),
        deleted: todo!(),
        user_id: todo!(),
        account_id: 0,
        bill_book_id: 0,
        bill_book_contents_id: 0,
        remark: todo!(),
    };
    diesel::insert_into(crate::model::diesel::fortune::fortune_schema::bill_record::table)
        .values(&bill_record_add)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
}
