#![allow(unused)]
#![allow(clippy::all)]

use rocket::serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::fortune::fortune_schema::*;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "bill_book_template_contents"]
pub struct BillBookTemplateContents {
    pub id: i64,
    pub parent_id: i64,
    pub created_time: i64,
    pub updated_time: i64,
    pub name: String,
    pub contents_type: i32,
    pub deleted: i32,
    pub hidden: i32,
    pub sort: i32,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "bill_record"]
pub struct BillRecordAdd {
    pub created_time: i64,
    pub updated_time: i64,
    pub deleted: i32,
    pub user_id: i64,
    pub account_id: i32,
    pub bill_book_id: i64,
    pub bill_book_contents_id: i64,
    pub remark: Option<String>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "bill_book"]
pub struct BillBookAdd {
    pub created_time: i64,
    pub updated_time: i64,
    pub deleted: i32,
    pub creator: i64,
    pub bill_book_template_id: i64,
    pub name: String
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[table_name = "bill_book_contents"]
pub struct BillBookContentAdd {
    pub created_time: i64,
    pub updated_time: i64,
    pub deleted: i32,
    pub creator: i64,
    pub bill_book_id: i64,
    pub name: String,
    pub contents: String,
    pub content_id: i64,
    pub parent_id: i64,
    pub contents_type: i32,
}