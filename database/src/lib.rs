pub mod db;
mod schema;
mod models;

use crate::models::Author;

pub trait DatabaseRepository {
    fn run_migrations(&self);
    fn get_author_ids_by_email(&self) -> Vec<Author>;
    fn get_authors_by_id(&self);
}

