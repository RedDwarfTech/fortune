use rocket::futures::StreamExt;
use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};
use rocket::serde::json::Json;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::model::diesel::fortune::fortune_custom_models::{BillBookAdd, BillBookContentAdd, BillBookTemplateContents, BillRecordAdd};

use crate::model::diesel::fortune::fortune_models::{BillBook, BillBookContent, BillBookTemplate, BillBookTemplateContent, BillRecord};
use crate::model::diesel::fortune::fortune_schema::bill_book::creator;
use crate::model::request::bill::bill_book_request::BillBookRequest;

pub fn get_bill_book_list(filter_name: Option<String>,login_user_info: &LoginUserInfo) -> Vec<BillBook> {
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    use crate::model::diesel::fortune::fortune_schema::bill_book as bill_book_table;
    let mut query = bill_book_table::table.into_boxed::<diesel::pg::Pg>();
    if let Some(some_filter_name) = &filter_name {
        query = query.filter(bill_book_table::name.like(format!("{}{}{}","%",some_filter_name.as_str(),"%")));
    }
    query = query.filter(bill_book_table::creator.eq(login_user_info.userId));
    let user_bill_books = query
        .load::<BillBook>(&connection)
        .expect("error get user bill book");
    return user_bill_books;
}

pub fn get_bill_book_by_id(filter_bill_book_id: &i64) -> BillBook{
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    use crate::model::diesel::fortune::fortune_schema::bill_book::dsl::*;
    let predicate = id.eq(filter_bill_book_id);
    let templates = bill_book
        .filter(predicate)
        .load::<BillBook>(&connection)
        .expect("error get user contents");
    return templates.get(0).unwrap().to_owned();
}

fn get_template_list_by_id(template_id: i64) -> Vec<BillBookTemplate>{
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    use crate::model::diesel::fortune::fortune_schema::bill_book_template::dsl::*;
    let predicate = id.eq(template_id);
    let templates = bill_book_template
        .filter(predicate)
        .load::<BillBookTemplate>(&connection)
        .expect("error get user contents");
    return templates;
}

fn get_template_list_count_by_user_id(filter_user_id: &i64) -> i64{
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
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
pub fn add_bill_book(request:&Json<BillBookRequest>, login_user_info: &LoginUserInfo) -> Result<BillBook,String> {
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let templates = get_template_list_by_id(request.billBookTemplateId);
    if templates.is_empty() {
        return Err("the template did not exists, check your template id first".parse().unwrap());
    }
    let templates_count = get_template_list_count_by_user_id(&login_user_info.userId);
    if templates_count >= 2 {
        return Err("2 bill book for every user".parse().unwrap());
    }
    return add_bill_book_impl(login_user_info, &templates, request);
}

///
/// 初始化账本数据
///
fn add_bill_book_impl(login_user_info: &LoginUserInfo, templates: &Vec<BillBookTemplate>, request:&Json<BillBookRequest>) -> Result<BillBook,String>{
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
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
    return Ok(r);
}

///
/// 初始化账本目录数据
///
fn add_bill_book_categories(template_id: i64,input_bill_book_id: i64, login_user_info: &LoginUserInfo){
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    use crate::model::diesel::fortune::fortune_schema::bill_book_template_contents::dsl::*;
    let predicate = id.eq(template_id);
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
            bill_book_id: input_bill_book_id,
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




