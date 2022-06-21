use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::fortune::fortune_custom_models::{BillBookAdd, BillBookContentAdd, BillBookRoleAdd};
use crate::model::diesel::fortune::fortune_models::{BillBook, BillBookAccount, BillBookTemplate, BillBookTemplateContent, Role};
use crate::model::request::bill::account::bill_account_request::BillAccountRequest;
use crate::model::request::bill::book::bill_book_request::BillBookRequest;
use crate::model::response::bill::bill_book_account_response::BillBookAccountResponse;
use crate::utils::database::get_connection;

pub fn get_bill_book_account_list(request: &BillAccountRequest) -> Vec<BillBookAccountResponse>{
    let bill_book_accounts = get_bill_account_list(request);
    let result = get_bill_book_account_sum(request).unwrap();
    let bill_book_account_resp = Vec::new();
    for bill_book_account in bill_book_accounts.iter() {
        let acc:Vec<(i64, i64)> = result.iter()
        .filter(|voc| voc.1 == bill_book_account.id)
        .cloned()
        .collect();
        if acc.is_empty() {
            let account_resp = BillBookAccountResponse{ 
                id: bill_book_account.id, 
                name: bill_book_account.name.to_string(), 
                icon_url: todo!(), 
                amount: 0, 
                account_type: todo!() 
            };
            bill_book_account_resp.push(account_resp);
        }
        else{
            let account_resp = BillBookAccountResponse{ 
                id: bill_book_account.id, 
                name: bill_book_account.name.to_string(), 
                icon_url: todo!(), 
                amount: 0, 
                account_type: todo!() 
            };
            bill_book_account_resp.push(account_resp);
        }
    }

    return bill_book_account_resp;
}

fn get_bill_account_list(request: &BillAccountRequest) -> Vec<BillBookAccount>{
    use crate::model::diesel::fortune::fortune_schema::bill_book_account as bill_book_account_table;
    let source_query = bill_book_account_table::table
        .filter(bill_book_account_table::dsl::bill_book_id.eq(request.bill_book_id));
    let result = source_query.load::<BillBookAccount>(&get_connection());
    return result.unwrap();
}

///
/// diesel聚合查询遇到的问题：
/// https://stackoverflow.com/questions/72670161/how-to-using-rust-diesel-to-do-the-group-by-query
/// https://github.com/diesel-rs/diesel/discussions/3209
/// 如何避免的方案：
/// https://github.com/diesel-rs/diesel/issues/1781
/// PostgreSQL里bigint求和返回的是numeric类型的数据
/// https://stackoverflow.com/questions/72675358/received-more-than-8-bytes-decoding-i64-was-an-expression-of-a-different-type-m
/// numeric类型的数据在diesel里用PgNumeric来接收
/// 但是不知道如何将PgNumeric的数据转换为BigDecimal
/// https://stackoverflow.com/questions/72676400/how-to-get-the-exactly-value-from-the-pgnumeric
pub fn get_bill_book_account_sum(request: &BillAccountRequest) -> Result<Vec<(i64, i64)>, diesel::result::Error>{
    use crate::diesel::GroupByDsl;
    use crate::model::diesel::fortune::fortune_schema::bill_record as bill_record_table;
    let source_query = bill_record_table::table
        .group_by(bill_record_table::account_id)
        .select((diesel::dsl::sql::<diesel::sql_types::BigInt>("SUM(CAST(amount AS Integer))"),bill_record_table::account_id))
        .filter(bill_record_table::dsl::bill_book_id.eq(request.bill_book_id));
    let result = source_query.load::<(i64,i64)>(&get_connection());
    return result;
}

pub fn get_bill_book_by_id(filter_bill_book_id: &i64) -> BillBook{
    let connection = get_connection();
    use crate::model::diesel::fortune::fortune_schema::bill_book::dsl::*;
    let predicate = id.eq(filter_bill_book_id);
    let templates = bill_book
        .filter(predicate)
        .load::<BillBook>(&connection)
        .expect("error get user contents");
    return templates.get(0).unwrap().to_owned();
}

fn get_template_list_by_id(template_id: i64) -> Vec<BillBookTemplate>{
    let connection = get_connection();
    use crate::model::diesel::fortune::fortune_schema::bill_book_template::dsl::*;
    let predicate = id.eq(template_id);
    let templates = bill_book_template
        .filter(predicate)
        .load::<BillBookTemplate>(&connection)
        .expect("error get user contents");
    return templates;
}

fn get_template_list_count_by_user_id(filter_user_id: &i64) -> i64{
    let connection = get_connection();
    use crate::model::diesel::fortune::fortune_schema::bill_book::dsl::*;
    let predicate = creator.eq(filter_user_id);
    let templates_count = bill_book
        .filter(predicate)
        .count()
        .get_result(&connection);
    return templates_count.unwrap_or(0);
}

///
/// 新增账本时，除了初始化账本数据
/// 还要初始化当前账本收入、支出等类型的目录数据
/// 不同的账本目录可自定义
///
pub fn add_bill_book(request:&Json<BillBookRequest>, login_user_info: &LoginUserInfo) -> Result<BillBook, String> {
    let connection = get_connection();
    let templates = get_template_list_by_id(request.billBookTemplateId);
    if templates.is_empty() {
        return Err("the template did not exists, check your template id first".parse().unwrap());
    }
    let templates_count = get_template_list_count_by_user_id(&login_user_info.userId);
    if templates_count >= 20 {
        return Err("2 bill book for every user".parse().unwrap());
    }
    let transaction_result = connection.build_transaction()
        .repeatable_read()
        .run::<_, diesel::result::Error, _>(||{
             return add_bill_book_impl(login_user_info, &templates, request);
        });
    return match transaction_result {
        Ok(v) => {
            Ok(v)
        },
        Err(_e) => {
            Err("database error".parse().unwrap())
        }
    };
}

///
/// 初始化账本数据
///
fn add_bill_book_impl(login_user_info: &LoginUserInfo, templates: &Vec<BillBookTemplate>, request:&Json<BillBookRequest>) -> Result<BillBook,diesel::result::Error>{
    let connection = get_connection();
    let bill_book_record = BillBookAdd{
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        deleted: 0,
        creator: login_user_info.userId,
        name: templates.get(0).unwrap().to_owned().name,
        bill_book_template_id: request.billBookTemplateId
    };
    let inserted_record = diesel::insert_into(crate::model::diesel::fortune::fortune_schema::bill_book::table)
        .values(&bill_book_record)
        .on_conflict_do_nothing()
        .get_results::<BillBook>(&connection);
    let records = inserted_record.unwrap();
    // 使用to_owned()表示重新拷贝了一份数据，和重新构建一个String出来别无二致
    let r = records.get(0).unwrap().to_owned();
    add_bill_book_categories(&r,login_user_info);
    add_bill_book_role(&r, login_user_info);
    return Ok(r);
}

///
/// 初始化账本目录数据
///
fn add_bill_book_categories(bill_book: &BillBook, login_user_info: &LoginUserInfo){
    let connection = get_connection();
    use crate::model::diesel::fortune::fortune_schema::bill_book_template_contents::dsl::*;
    let predicate = bill_book_template_id.eq(bill_book.bill_book_template_id);
    let categories_record = bill_book_template_contents
        .filter(predicate)
        .load::<BillBookTemplateContent>(&connection)
        .expect("error get categories contents");
    let mut bill_book_contents:Vec<BillBookContentAdd> = Vec::new();
    for record in categories_record {
        let bill_book_content = BillBookContentAdd{
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            deleted: 0,
            creator: login_user_info.userId,
            bill_book_id: bill_book.id,
            name: record.name,
            contents: "".to_string(),
            content_id: record.id,
            parent_id: record.parent_id,
            contents_type: record.contents_type
        };
        bill_book_contents.push(bill_book_content);
    }
    diesel::insert_into(crate::model::diesel::fortune::fortune_schema::bill_book_contents::table)
        .values(&bill_book_contents)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
}

///
/// 初始化账本角色数据
///
fn add_bill_book_role(bill_book: &BillBook, login_user_info: &LoginUserInfo){
    let connection = get_connection();
    use crate::model::diesel::fortune::fortune_schema::role::dsl::*;
    let predicate = role_type.eq(1);
    let categories_record = role
        .filter(predicate)
        .load::<Role>(&connection)
        .expect("error get categories contents");
    let mut bill_book_roles:Vec<BillBookRoleAdd> = Vec::new();
    for record in categories_record {
        let bill_book_content = BillBookRoleAdd{
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            deleted: 0,
            creator: login_user_info.userId,
            bill_book_id: bill_book.id,
            remark: Option::from(record.remark),
            name: record.name,
            role_type: record.role_type
        };
        bill_book_roles.push(bill_book_content);
    }
    diesel::insert_into(crate::model::diesel::fortune::fortune_schema::bill_book_role::table)
        .values(&bill_book_roles)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
}

