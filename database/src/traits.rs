// pub mod posts;
pub mod author;
pub mod dao;
pub mod models;
pub mod posts;
mod schema;
use async_trait::async_trait;

use dao::DB;
use diesel::{r2d2::ConnectionManager, PgConnection};
use models::Post;

use crate::models::{Author, PostById};

#[async_trait]
pub trait DAO {
    async fn new() -> DB;
    async fn get_connection(
        &self,
    ) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error>;
}

#[async_trait]
pub trait AuthorDAO {
    async fn get_author_ids_by_email(
        &self,
        mail: &Vec<String>,
    ) -> Result<Vec<i32>, Box<dyn std::error::Error>>;
    async fn get_authors_by_ids(
        &self,
        author_ids: &Vec<i32>,
    ) -> Result<Vec<Author>, Box<dyn std::error::Error>>;
}

#[async_trait]
#[async_trait(?Send)]
pub trait PostDAO {
    async fn get_posts_by_ids(
        &self,
        post_ids: &Vec<i32>,
    ) -> Result<Vec<PostById>, Box<dyn std::error::Error>>;
    async fn upsert_post(
        &self,
        post: Post,
        author_ids: Vec<i32>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
