use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::diesel::fortune::fortune_custom_models::BillRecordUpdate;
use crate::model::request::bill::bill_del_request::BillDelRequest;
use crate::model::request::bill::bill_edit_request::BillEditRequest;
use crate::model::request::bill::book::bill_book_archive_request::BillBookArchiveRequest;
use crate::model::{request::bill::bill_add_request::BillAddRequest, diesel::fortune::fortune_custom_models::BillRecordAdd};
use crate::model::diesel::fortune::fortune_models::{BillBook, BillRecord};
use crate::model::diesel::fortune::fortune_schema::bill_book::archived;
use crate::model::diesel::fortune::fortune_schema::bill_book::dsl::bill_book;
use crate::model::request::bill::bill_detail_request::BillDetailRequest;
use crate::model::request::bill::bill_page_request::BillPageRequest;
use crate::utils::database::get_connection;

pub fn add_bill(_request: Json<BillAddRequest>, login_user_info: &LoginUserInfo) -> Result<BillRecord, String> {
    let connection = get_connection();
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

pub fn query_bill_records(_request: &BillPageRequest) -> Vec<BillRecord>{
    use crate::model::diesel::fortune::fortune_schema::bill_record as bill_record_table;
    use crate::diesel::BoolExpressionMethods;
    let predicate = bill_record_table::dsl::bill_book_id.eq(_request.bill_book_id)
    .and(bill_record_table::dsl::deleted.eq(0));
    let bill_records = bill_record_table::table
    .filter(predicate)
    .get_results::<BillRecord>(&get_connection())
    .expect("get bill records failed");
    return bill_records;
}

pub fn query_recoverable_records(query: &BillPageRequest) -> Vec<BillRecord>{
    use crate::model::diesel::fortune::fortune_schema::bill_record as bill_record_table;
    use crate::diesel::BoolExpressionMethods;
    let predicate = bill_record_table::dsl::deleted.eq(1)
    .and(bill_record_table::dsl::bill_book_id.eq(query.bill_book_id));
    let bill_book_records = bill_record_table::table
    .filter(predicate)
    .limit(query.pageSize)
    .offset(query.pageSize * query.pageNum)
    .load::<BillRecord>(&get_connection())
    .expect("get bill records failed");
    return bill_book_records;
}

pub fn recover_bill_record(query: &BillPageRequest){
    let connection = get_connection();
    use crate::model::diesel::fortune::fortune_schema::bill_book as bill_book_table;
    let predicate = bill_book_table::id.eq(query.bill_book_id);
    diesel::update(bill_book.filter(predicate))
        .set((archived.eq(1)))
        .execute(&connection)
        .expect("unable to update bill book contents");
}

pub fn query_bill_record_detail(_request: &BillDetailRequest) -> BillRecord {
    let connection = get_connection();
    use crate::model::diesel::fortune::fortune_schema::bill_record as bill_record_table;
    let predicate = bill_record_table::id.eq(_request.id);
    let bill_record_result = bill_record_table::table
        .filter(predicate)
        .get_result::<BillRecord>(&connection);
    return bill_record_result.unwrap();
}

pub fn del_bill_record(_request: &BillDelRequest, login_user_info: &LoginUserInfo) {
    use crate::diesel::BoolExpressionMethods;
    let connection = get_connection();
    use crate::model::diesel::fortune::fortune_schema::bill_record as bill_record_table;
    let predicate = bill_record_table::id.eq(_request.id)
    .and(bill_record_table::user_id.eq(login_user_info.userId));
    diesel::delete(bill_record_table::table.filter(predicate)).execute(&connection);
}

pub fn edit_bill_record(request: &BillEditRequest, _login_user_info: &LoginUserInfo) {
    let connection = get_connection();
    use crate::model::diesel::fortune::fortune_schema::bill_record as bill_record_table;
    let predicate = bill_record_table::dsl::id.eq(request.id);

    // https://diesel.rs/guides/all-about-updates.html
    // https://stackoverflow.com/questions/72249171/rust-diesel-conditionally-update-fields
    diesel::update(bill_record_table::table.filter(predicate))
        .set(&BillRecordUpdate{
            amount: Some(request.amount),
        })
        .get_result::<BillRecord>(&connection)
        .expect("unable to update channel");
}

pub fn archive_bill_book(_request: &BillBookArchiveRequest) {
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    use crate::model::diesel::fortune::fortune_schema::bill_book as bill_book_table;
    let predicate = bill_book_table::id.eq(_request.bill_book_id);
    diesel::update(bill_book.filter(predicate))
        .set((archived.eq(1)))
        .execute(&connection)
        .expect("unable to update bill book contents");
}