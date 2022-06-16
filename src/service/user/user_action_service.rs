use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rust_wheel::config::db::config;

use crate::model::diesel::fortune::fortune_models::UserActionLog;
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
