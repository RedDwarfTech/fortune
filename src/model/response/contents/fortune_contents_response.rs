use rust_wheel::common::util::convert_to_tree_i64::IntoTree;
use serde::Serialize;
use crate::model::diesel::fortune::fortune_custom_models::BillBookTemplateContents;
use crate::model::diesel::fortune::fortune_models::BillBookContent;

#[derive(Debug, Serialize)]
pub struct FortuneContentResponse {
    pub id: i64,
    pub parent_id: i64,
    pub created_time: i64,
    pub updated_time: i64,
    pub name: String,
    pub contents_type: i32,
    pub deleted: i32,
    pub hidden: i32,
    pub sort: i32,
    pub children: Vec<FortuneContentResponse>
}

/// https://stackoverflow.com/questions/72569425/borrowed-value-does-not-live-long-enough-when-write-an-generic-object-mapping-fu
impl From<&BillBookContent> for FortuneContentResponse {
    fn from(p: &BillBookContent) -> Self {
        Self {
            id: p.id,
            parent_id: p.parent_id,
            created_time: p.created_time,
            updated_time: p.updated_time,
            name: p.name.to_string(),
            contents_type: p.contents_type,
            deleted: p.deleted,
            hidden: 0,
            sort: 0,
            children: vec![]
        }
    }
}

impl IntoTree for &FortuneContentResponse {
    type Output = FortuneContentResponse;

    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_parent_id(&self) -> i64 {
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
            sort: self.sort,
        }
    }
}
