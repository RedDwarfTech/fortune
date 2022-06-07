/**
To make the clion show unused imports

go to Settings > Editor > Inspections > Rust > Lints > Unused Import, enable it, and now CTRL+ALT+O will remove unused imports!

https://stackoverflow.com/questions/61077692/how-can-i-fix-unused-imports-in-rust-automatically
**/

use rocket::response::content;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::service::bill::bill_service::add_bill;
use crate::model::request::bill::bill_request::BillRequest;

#[get("/v1/add")]
pub fn list(request: Json<BillRequest>) -> content::RawJson<String> {
    let contents = add_bill(request);
    return box_rest_response(contents);
}

