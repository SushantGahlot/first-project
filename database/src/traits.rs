// pub mod posts;
pub mod author;
pub mod dao;
pub mod models;
pub mod posts;
mod schema;

use dao::DB;
use diesel::{r2d2::ConnectionManager, PgConnection};
use models::Post;

use crate::models::{Author, PostById};

pub trait DAO {
    fn new() -> DB;
    fn get_connection(
        &self,
    ) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error>;
}

pub trait AuthorDAO {
    fn get_author_ids_by_email(
        &self,
        mail: &Vec<String>,
    ) -> Result<Vec<i32>, Box<dyn std::error::Error>>;
    fn get_authors_by_ids(
        &self,
        author_ids: &Vec<i32>,
    ) -> Result<Vec<Author>, Box<dyn std::error::Error>>;
}

pub trait PostDAO {
    fn get_posts_by_ids(
        &self,
        post_ids: &Vec<i32>,
    ) -> Result<Vec<PostById>, Box<dyn std::error::Error>>;
    fn upsert_post(
        &self,
        post: Post,
        author_ids: Vec<i32>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
