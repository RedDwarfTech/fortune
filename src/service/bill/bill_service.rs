use diesel::RunQueryDsl;
use rocket::serde::json::Json;
use rust_wheel::config::db::config;

use crate::model::{request::bill::bill_request::BillRequest, diesel::fortune::fortune_custom_models::BillRecordAdd};

pub fn add_bill(_request: Json<BillRequest>){
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let bill_record_add = BillRecordAdd{
        created_time: todo!(),
        updated_time: todo!(),
        deleted: todo!(),
        user_id: todo!(),
        contents_id: todo!(),
        remark: todo!(),
    };
    diesel::insert_into(crate::model::diesel::fortune::fortune_schema::bill_record::table)
    .values(&bill_record_add)
    .on_conflict_do_nothing()
    .execute(&connection)
    .unwrap();
}

