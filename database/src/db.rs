use backoff::ExponentialBackoff;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use std::time::Duration;
use crate::schema::author::{self, email};
use crate::models::Author;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::DatabaseRepository;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct DBRepository {}


fn establish_connection() -> PgConnection {
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let op = || PgConnection::establish(&database_url).map_err(backoff::Error::transient);

    let backoff = ExponentialBackoff {
        max_elapsed_time: Some(Duration::from_secs(60)),
        ..Default::default()
    };

    return backoff::retry(backoff, op).expect("could not establish connection with the database");
}


impl DatabaseRepository for DBRepository {
    fn run_migrations(&self) {
        let connection = &mut establish_connection();
    
        connection.run_pending_migrations(MIGRATIONS).expect("failed running migrations");
    }

    fn get_author_ids_by_email(&self) -> Vec<Author> {
        let connection: &mut PgConnection = &mut establish_connection();
        let results = author::table.filter(email.eq("millerbrian@example.com")).load::<Author>(connection).expect("error loading authors");
        // let results = author.filter(email.eq("mail")).load::<Author>.expect("error loading authors");

        return results;
    }

    fn get_authors_by_id(&self) {
        todo!()
    }
}