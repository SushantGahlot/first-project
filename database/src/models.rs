use diesel::{prelude::*, sql_types::{Integer, Text, Array, Timestamp}};
use chrono::NaiveDateTime;
use crate::schema::*;


#[derive(Queryable, Debug, Identifiable, Insertable)]
#[diesel(table_name = author)]
#[diesel(primary_key(authorid))]
pub struct Author {
    pub authorid: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Queryable, Debug, Identifiable, AsChangeset, Insertable)]
#[diesel(table_name = post)]
#[diesel(primary_key(postid))]
pub struct Post {
    pub body: Option<String>,
    pub created: Option<NaiveDateTime>,
    pub postid: Option<i32>,
    pub title: String,
    pub updated: Option<NaiveDateTime>,
}

#[derive(Queryable, Identifiable, Associations, Insertable)]
#[diesel(table_name = author_post)]
#[diesel(belongs_to(Author, foreign_key = authorid))]
#[diesel(belongs_to(Post, foreign_key = postid))]
pub struct AuthorPost {
    pub id: Option<i32>,
    pub authorid: i32,
    pub postid: i32,
}

#[derive(QueryableByName, Debug)]
pub struct PostById {
    #[diesel(sql_type = Integer)]
    pub postid: i32,
    #[diesel(sql_type = Text)]
    pub title: String,
    #[diesel(sql_type = Text)]
    pub body: String,
    #[diesel(sql_type = Timestamp)]
    pub updated: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub created: NaiveDateTime,
    #[diesel(sql_type = Array<Integer>)]
    pub authorids: Vec<i32>,
    #[diesel(sql_type = Array<Text>)]
    pub firstnames: Vec<String>,
    #[diesel(sql_type = Array<Text>)]
    pub lastnames: Vec<String>,
    #[diesel(sql_type = Array<Text>)]
    pub usernames: Vec<String>,
    #[diesel(sql_type = Array<Text>)]
    pub emails: Vec<String>,
}