use diesel::{BoolExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::dsl::{any, not};
use rust_wheel::common::util::convert_to_tree::convert_to_tree;
use rust_wheel::config::db::config;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use crate::diesel::ExpressionMethods;

///
/// 每个用户看到的菜单都不一样
/// 所以先查询当前用户拥有的菜单ID集合
/// 再组装成当前用户的菜单树
///
/// 针对系统菜单的增减
/// 后期用户可以在界面上进行区分
/// 会显示2个区域，一个是已经有的记账类型，一个是目前可以加入的记账类型
///
pub fn content_tree_query<T>(filter_content_type: i32, login_user_info: LoginUserInfo) {

}
