use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rocket::futures::TryFutureExt;
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::{request::bill::bill_add_request::BillAddRequest, diesel::fortune::fortune_custom_models::BillRecordAdd};
use crate::model::diesel::fortune::fortune_models::{BillBook, BillRecord, Role};
use crate::model::diesel::fortune::fortune_schema::bill_book::archived;
use crate::model::diesel::fortune::fortune_schema::bill_book::dsl::bill_book;
use crate::model::request::bill::bill_book_archive_request::BillBookArchiveRequest;
use crate::model::request::bill::bill_detail_request::BillDetailRequest;
use crate::model::request::bill::bill_page_request::BillPageRequest;
use crate::model::request::role::role_list_request::RoleListRequest;

pub fn add_role(_request: Json<BillAddRequest>, login_user_info: &LoginUserInfo) -> Result<BillRecord, String> {
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    use crate::model::diesel::fortune::fortune_schema::bill_book as bill_book_table;
    let predicate = bill_book_table::id.eq(_request.bill_book_id);
    let bill_book_records = bill_book_table::table
        .filter(predicate)
        .load::<BillBook>(&connection)
        .expect("get bill book failed");
    if bill_book_records.is_empty() {
        return Err("not found bill book".parse().unwrap());
    }
    if bill_book_records.get(0).unwrap().archived.to_owned() == 1 {
        return Err("bill book archived".parse().unwrap());
    }
    let bill_record_add = BillRecordAdd {
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        deleted: 0,
        user_id: login_user_info.userId,
        account_id: _request.account_id,
        bill_book_id: _request.bill_book_id,
        bill_book_contents_id: _request.bill_book_contents_id,
        remark: _request.remark.to_owned(),
    };
    let insert_result = diesel::insert_into(crate::model::diesel::fortune::fortune_schema::bill_record::table)
        .values(&bill_record_add)
        .get_result::<BillRecord>(&connection);
    Ok(insert_result.unwrap())
}

pub fn query_bill_book_roles(query: &RoleListRequest, login_user_info: &LoginUserInfo) -> Vec<Role> {
    use crate::model::diesel::fortune::fortune_schema::role as role_table;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let role_results = role_table::table
        .get_results::<Role>(&connection)
        .expect("get results failed");
    return role_results;
}