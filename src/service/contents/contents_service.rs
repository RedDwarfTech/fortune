use diesel::{BoolExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::dsl::{any, not};
use rocket::futures::future::err;
use rocket::serde::json::Json;
use rust_wheel::common::util::convert_to_tree_i64::convert_to_tree;
use rust_wheel::common::util::model_convert::map_entity;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::diesel::ExpressionMethods;
use crate::model::diesel::fortune::fortune_custom_models::BillBookContentAdd;
use crate::model::diesel::fortune::fortune_models::BillBookContent;
use crate::model::diesel::fortune::fortune_schema::bill_book_template_contents::parent_id;
use crate::model::diesel::fortune::fortune_schema::bill_record::bill_book_id;
use crate::model::request::contents::add_contents_request::AddContentsRequest;
use crate::model::request::contents::del_contents_request::DelContentsRequest;
use crate::model::request::contents::edit_contents_request::EditContentsRequest;
use crate::model::response::contents::fortune_contents_response::FortuneContentResponse;

///
/// 每个用户看到的菜单都不一样
/// 所以先查询当前用户拥有的菜单ID集合
/// 再组装成当前用户的菜单树
///
/// 针对系统菜单的增减
/// 后期用户可以在界面上进行区分
/// 会显示2个区域，一个是已经有的记账类型，一个是目前可以加入的记账类型
///
pub fn content_tree_query(filter_content_type: i32, filter_book_id: i64) -> Vec<FortuneContentResponse> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_contents::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let predicate = bill_book_id.eq(filter_book_id).and(contents_type.eq(filter_content_type));
    let contents_result = bill_book_contents.filter(&predicate)
        .load::<BillBookContent>(&connection)
        .expect("Error fortune contents resource");
    let response = map_entity(contents_result);
    return convert_to_tree_impl(&response);
}

fn convert_to_tree_impl(contents: &Vec<FortuneContentResponse>) -> Vec<FortuneContentResponse> {
    let mut root_element: Vec<_> = contents.iter()
        .filter(|content| content.parent_id == 0)
        .collect();
    let mut sub_element: Vec<_> = contents.iter()
        .filter(|content| content.parent_id != 0)
        .collect();
    let result = convert_to_tree(&root_element, &sub_element);
    return result;
}

pub fn add_book_contents(request: &Json<AddContentsRequest>, login_user_info: &LoginUserInfo) -> Result<BillBookContent, String> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_contents::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let predicate = bill_book_id.eq(request.bill_book_id)
        .and(parent_id.eq(request.parent_id))
        .and(name.eq(request.name.to_string()));
    let contents_result = bill_book_contents.filter(&predicate)
        .load::<BillBookContent>(&connection)
        .expect("Error fortune contents resource");
    if !contents_result.is_empty() {
        return Err("相同名称已经存在").expect("TODO: panic message");
    }
    let book_content = BillBookContentAdd {
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        deleted: 0,
        creator: login_user_info.userId,
        bill_book_id: request.bill_book_id,
        name: request.name.to_string(),
        contents: "".to_string(),
        content_id: 0,
        parent_id: request.parent_id,
        contents_type: 0,
    };
    let inserted_contents = diesel::insert_into(crate::model::diesel::fortune::fortune_schema::bill_book_contents::table)
        .values(&book_content)
        .get_results::<BillBookContent>(&connection);
    return if inserted_contents.as_ref().unwrap().is_empty() {
        Err("insert 0 record".parse().unwrap())
    } else {
        let inserted_record = inserted_contents.unwrap().get(0).unwrap().to_owned();
        update_contents_id(&inserted_record.id);
        Ok(inserted_record)
    }
}

/// 关于新增自定义分类时，分类的ID选择问题，目前想到的方案是直接使用账本与账本分类关系表的ID
/// 但是选择此ID可能会导致ID冲突，就是选择的ID已经存在于现有ID
/// 也可以采用雪花算法生成分布式ID
/// 目前没有成熟稳定的库
/// 可以采用UUID，但与现有的存储不匹配，且占据空间，检索性能相对较低
fn update_contents_id(pk_id: &i64){
    use crate::model::diesel::fortune::fortune_schema::bill_book_contents::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let predicate = id.eq(pk_id);
    diesel::update(bill_book_contents.filter(predicate))
        .set((content_id.eq(pk_id)))
        .execute(&connection)
        .expect("unable to update bill book contents");
}

pub fn del_book_contents<'a>(request: &'a Json<DelContentsRequest>, login_user_info: &'a LoginUserInfo) -> Result<String, &'a str> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_contents::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let predicate = id.eq(any(&request.ids));
    let contents_result = bill_book_contents.filter(&predicate)
        .load::<BillBookContent>(&connection)
        .expect("Error fortune contents resource");
    if contents_result.is_empty() {
        // return Ok("ok");
    }
    for bill_book in contents_result {
        if bill_book.creator != login_user_info.userId {
            return Err("not contents creator");
        }
    }
    let transaction_result = connection.build_transaction()
        .repeatable_read()
        .run::<_, diesel::result::Error, _>(||{
            delete_bill_records(&request.ids, &request.bill_book_id).expect("TODO: panic message");
            return delete_contents(&request.ids, &request.bill_book_id);
        });
    return match transaction_result {
        Ok(v) => {
            Ok(v)
        },
        Err(_e) => {
            Err("database error")
        }
    };
}

fn delete_contents(ids: &Vec<i64>, filter_bill_book_id: &i64) -> Result<String, diesel::result::Error> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_contents::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let predicate = id.eq(any(&ids)).and(bill_book_id.eq(filter_bill_book_id));
    let delete_result = diesel::delete(bill_book_contents.filter(predicate))
        .execute(&connection);
    Ok("ok".parse().unwrap())
}

fn delete_bill_records(ids: &Vec<i64>, filter_bill_book_id: &i64) -> Result<String,diesel::result::Error> {
    use crate::model::diesel::fortune::fortune_schema::bill_record::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let predicate = bill_book_contents_id.eq(any(&ids)).and(bill_book_id.eq(filter_bill_book_id));
    let delete_result = diesel::delete(bill_record.filter(predicate)).execute(&connection);
    Ok("ok".parse().unwrap())
}

pub fn edit_book_contents<'a>(request: &'a Json<EditContentsRequest>, login_user_info: &'a LoginUserInfo) -> Result<String, &'a str> {
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let transaction_result = connection.build_transaction()
        .repeatable_read()
        .run::<_, diesel::result::Error, _>(||{
            return edit_bill_contents(&request.id, &request.bill_book_id, &request.name);
        });
    return match transaction_result {
        Ok(v) => {
            Ok(v)
        },
        Err(_e) => {
            Err("database error")
        }
    };
}

fn edit_bill_contents(filter_id: &i64, filter_bill_book_id: &i64, new_name: &String) -> Result<String,diesel::result::Error> {
    use crate::model::diesel::fortune::fortune_schema::bill_book_contents::dsl::*;
    let connection = config::connection("FORTUNE_DATABASE_URL".to_string());
    let predicate = id.eq(filter_id).and(bill_book_id.eq(filter_bill_book_id));
    diesel::update(bill_book_contents.filter(predicate))
        .set((name.eq(new_name)))
        .execute(&connection)
        .expect("unable to update bill book contents");
    Ok("ok".parse().unwrap())
}