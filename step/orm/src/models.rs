

use super::schema::posts;

// 用于查询
#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
}