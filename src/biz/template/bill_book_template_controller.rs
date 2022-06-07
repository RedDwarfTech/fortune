/**
To make the clion show unused imports

go to Settings > Editor > Inspections > Rust > Lints > Unused Import, enable it, and now CTRL+ALT+O will remove unused imports!

https://stackoverflow.com/questions/61077692/how-can-i-fix-unused-imports-in-rust-automatically
**/

use rocket::response::content;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::service::template::bill_book_template_service::get_template_list;

#[get("/v1/list")]
pub fn list() -> content::RawJson<String> {
    let contents = get_template_list();
    return box_rest_response(contents);
}

