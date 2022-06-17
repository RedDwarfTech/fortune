use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::request::role::add_role_request::AddRoleRequest;
use crate::model::diesel::fortune::fortune_custom_models::BillBookRoleAdd;
use crate::model::diesel::fortune::fortune_models::{BillBook, BillBookRole};
use crate::model::request::role::bill_book_role_request::BillBookRoleRequest;
use crate::model::request::role::role_list_request::RoleListRequest;
use crate::utils::database::get_connection;

pub fn query_bill_book_roles(query: &RoleListRequest, _login_user_info: &LoginUserInfo) -> Vec<BillBookRole> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_role as bill_book_role_table;
    let connection = get_connection();
    let predicate = bill_book_role_table::dsl::bill_book_id.eq(query.bill_book_id);
    let role_results = bill_book_role_table::table
        .filter(predicate)
        .get_results::<BillBookRole>(&connection)
        .expect("get results failed");
    return role_results;
}

pub fn query_bill_book_roles_detail(query: &BillBookRoleRequest, _login_user_info: &LoginUserInfo) -> BillBookRole {
    use crate::model::diesel::fortune::fortune_schema::bill_book_role as bill_book_role_table;
    let connection = get_connection();
    let predicate = bill_book_role_table::dsl::id.eq(query.id);
    let role_results = bill_book_role_table::table
        .filter(predicate)
        .get_result::<BillBookRole>(&connection)
        .expect("get results failed");
    return role_results;
}

pub fn add_bill_book_role(query: &Json<AddRoleRequest>, login_user_info: &LoginUserInfo) -> Result<BillBookRole, String> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_role as bill_book_role_table;
    use crate::model::diesel::fortune::fortune_schema::bill_book as bill_book_table;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let predicate = bill_book_table::dsl::id.eq(query.bill_book_id);
    let bill_book_record = bill_book_table::table
        .filter(predicate)
        .get_results::<BillBook>(&connection)
        .expect("get bill book error");
    if bill_book_record.is_empty() {
        return Err("not found bill book".parse().unwrap());
    }
    if bill_book_record.get(0).unwrap().creator != login_user_info.userId {
        return Err("only owner could add role".parse().unwrap());
    }
    let bill_book_role_add = BillBookRoleAdd{
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        deleted: 0,
        creator: login_user_info.userId,
        bill_book_id: query.bill_book_id,
        remark: query.remark.to_owned(),
        role_type: 2,
        name: query.name.to_string()
    };
    let inserted_result = diesel::insert_into(bill_book_role_table::table)
        .values(&bill_book_role_add)
        .get_result::<BillBookRole>(&connection);
    return Ok(inserted_result.unwrap());
}