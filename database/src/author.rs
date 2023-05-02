use crate::models::Author;
use crate::schema::author::{self, authorid, email};
use crate::AuthorDAO;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use async_trait::async_trait;

pub struct AuthorDB {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait]
impl AuthorDAO for AuthorDB {
    async fn get_author_ids_by_email(
        &self,
        mail: &Vec<String>,
    ) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
        if mail.len() == 0 {
            Err("author email IDs can not be empty")?;
        }
        let mut conn = self.pool.get()?;
        let results = author::table
            .select(authorid)
            .filter(email.eq_any(mail))
            .load::<i32>(&mut conn)?;

        Ok(results)
    }

    async fn get_authors_by_ids(
        &self,
        author_ids: &Vec<i32>,
    ) -> Result<Vec<Author>, Box<dyn std::error::Error>> {
        if author_ids.len() == 0 {
            Err("author IDs can not be empty")?;
        }
        let mut conn = self.pool.get()?;
        let results = author::table
            .filter(authorid.eq_any(author_ids))
            .load::<Author>(&mut conn)?;

        Ok(results)
    }
}
