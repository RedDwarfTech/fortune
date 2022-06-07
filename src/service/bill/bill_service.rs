use diesel::RunQueryDsl;
use rust_wheel::config::db::config;

use crate::model::diesel::fortune::fortune_models::BillBookTemplate;
use crate::model::request::bill::bill_request::BillRequest;

pub fn add_bill(request: Json<BillRequest>){
    use crate::model::diesel::fortune::fortune_scema::bill_record::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    diesel::insert_into(crate::model::diesel::dolphin::dolphin_schema::bill_record::table)
    .values(&app)
    .on_conflict_do_nothing()
    .execute(&connection)
    .unwrap();
}

