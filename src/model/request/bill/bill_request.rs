use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BillRequest {
    pub billBookId: i64,
    pub contentsId: i64,
    pub amount: i64
}