use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::{ diesel::fortune::fortune_custom_models::BillRecordAdd };
use crate::model::diesel::fortune::fortune_models::UserActionLog;
use crate::model::diesel::fortune::fortune_schema::bill_book::dsl::bill_book;
use crate::model::request::user::user_action_request::UserActionRequest;

pub fn query_user_actions(_request: &UserActionRequest) -> Vec<UserActionLog> {
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    use crate::model::diesel::fortune::fortune_schema::user_action_log as action_log_table;
    let predicate = action_log_table::dsl::bill_book_id.eq(_request.bill_book_id);
    let action_results = action_log_table::table
        .filter(predicate)
        .limit(_request.limit)
        .offset(_request.offset)
        .get_results::<UserActionLog>(&connection)
        .expect("get action log error");
    return action_results;
}
