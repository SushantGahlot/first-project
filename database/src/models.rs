use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Author {
    pub authorid: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String
}