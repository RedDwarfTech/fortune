use rust_wheel::common::util::convert_to_tree::IntoTree;
use serde::Serialize;
use crate::model::diesel::fortune::fortune_models::FortuneContent;

#[derive(Debug, Serialize)]
pub struct FortuneContentResponse {
    pub id: i32,
    pub parent_id: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub name: String,
    pub contents_type: i32,
    pub deleted: i32,
    pub hidden: i32,
    pub sort: i32,
    pub children: Vec<FortuneContentResponse>,
}

impl IntoTree for &FortuneContent {
    type Output = FortuneContentResponse;

    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_parent_id(&self) -> i32 {
        self.parent_id
    }

    fn convert(&self, children: Vec<Self::Output>) -> Self::Output {
        FortuneContentResponse {
            id: self.id,
            parent_id: self.parent_id,
            created_time: self.created_time,
            updated_time: self.updated_time,
            name: self.name.to_string(),
            contents_type: self.contents_type,
            deleted: self.deleted,
            hidden: self.hidden,
            children,
            sort: self.sort
        }
    }
}



