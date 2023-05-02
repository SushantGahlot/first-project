extern crate backoff;
extern crate fake;
extern crate glob;
use chrono::TimeZone;
use diesel_migrations::MigrationHarness;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
use backoff::ExponentialBackoff;
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use fake::faker::chrono::en::DateTimeBetween;
use fake::faker::internet::raw::{SafeEmail, Username};
use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::faker::name::raw::{FirstName, LastName};
use fake::locales::EN;
use fake::Fake;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::env;
use std::time::Duration;

fn establish_connection() -> PgConnection {
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let op = || PgConnection::establish(&database_url).map_err(backoff::Error::transient);

    let backoff = ExponentialBackoff {
        max_elapsed_time: Some(Duration::from_secs(60)),
        ..Default::default()
    };

    return backoff::retry(backoff, op).expect("could not establish connection with the database");
}

fn seed_fake_data(mut conn: Vec<&mut PgConnection>) {
    use diesel::sql_types::Integer;
    let num_authors = 300;
    let num_posts = 30000;

    for con in conn.iter_mut() {
        sql_query("DELETE FROM author")
            .execute(*con)
            .expect("failed clearing author table");
        sql_query("ALTER SEQUENCE author_authorid_seq RESTART WITH 1")
            .execute(*con)
            .expect("failed resetting author pk sequence");
        sql_query("DELETE FROM post")
            .execute(*con)
            .expect("failed clearing post table");
        sql_query("ALTER SEQUENCE post_postid_seq RESTART WITH 1")
            .execute(*con)
            .expect("failed resetting post pk sequence");
        sql_query("DELETE FROM author_post")
            .execute(*con)
            .expect("failed clearing author_post table");
        sql_query("ALTER SEQUENCE author_post_id_seq RESTART WITH 1")
            .execute(*con)
            .expect("failed resetting author_post pk sequence");
    }

    // =====================================
    // ------------ author table -----------
    // =====================================
    let mut authors: Vec<String> = Vec::new();

    let mut author_emails = HashSet::new();
    while author_emails.len() < num_authors {
        author_emails.insert(SafeEmail(EN).fake::<String>());
    }

    for email in author_emails.iter() {
        let author = format!(
            "('{}', '{}', '{}', '{}')",
            Username(EN).fake::<String>().replace("'", ""),
            FirstName(EN).fake::<String>().replace("'", ""),
            LastName(EN).fake::<String>().replace("'", ""),
            email,
        );

        authors.push(author)
    }

    #[derive(QueryableByName, Clone)]
    struct InsertedAuthorId {
        #[diesel(sql_type = Integer)]
        authorid: i32,
    }

    println!("Inserting authors");
    let query = format!(
        "INSERT INTO author (username, firstname, lastname, email) VALUES {} RETURNING authorid",
        authors.join(",")
    );

    let mut authorids: Vec<InsertedAuthorId> = vec![];
    for con in conn.iter_mut() {
        authorids = sql_query(query.clone())
            .get_results(*con)
            .expect("error inserting authors");
    }

    // =====================================
    // ------------ post table -----------
    // =====================================
    let mut posts: Vec<String> = Vec::new();
    let dates = DateTimeBetween(
        chrono::Utc.with_ymd_and_hms(2018, 3, 17, 0, 0, 0).unwrap(),
        chrono::offset::Utc::now(),
    );

    for _ in 0..num_posts {
        let post = format!(
            "('{}', '{}', '{}', '{}')",
            Sentence(3..5).fake::<String>().replace("'", ""),
            Paragraph(3..7).fake::<String>().replace("'", ""),
            dates.fake::<DateTime<Utc>>().to_string(),
            dates.fake::<DateTime<Utc>>().to_string(),
        );

        posts.push(post)
    }

    #[derive(QueryableByName, Clone)]
    struct InsertedPostId {
        #[diesel(sql_type = Integer)]
        postid: i32,
    }

    println!("Inserting posts");
    let query = format!(
        "INSERT INTO post (title, body, created, updated) VALUES {} RETURNING postid",
        posts.join(",")
    );

    let mut postids: Vec<InsertedPostId> = vec![];
    for con in conn.iter_mut() {
        postids = sql_query(query.clone())
            .get_results(*con)
            .expect("error inserting posts");
    }

    // =====================================
    // --------- author_post table ---------
    // =====================================
    let mut rng = thread_rng();
    let mut authorids = authorids.clone();
    authorids.shuffle(&mut rng); // shuffle the author ids randomly
    let groups: Vec<_> = authorids.chunks(5).collect(); // divide into 60 groups of 5
    let mut author_post = Vec::new();

    for postid in postids {
        let group = groups.choose(&mut rng).unwrap(); // choose a random group for each post
        for authorid in group.iter() {
            author_post.push((authorid, postid.clone())); // create pairs with the group members
        }
    }

    let mut author_posts: Vec<String> = Vec::new();

    for i in author_post {
        author_posts.push(format!("('{}','{}')", i.0.authorid, i.1.postid));
    }

    println!("Inserting author posts");
    let query: String = format!(
        "INSERT INTO author_post (authorid, postid) VALUES {}",
        author_posts.join(",")
    );

    for con in conn.iter_mut() {
        sql_query(query.clone())
            .execute(*con)
            .expect("inserting in author_post table failed");
    }
}

fn main() {
    let mut conns: Vec<&mut PgConnection> = vec![];

    let rust_db = &mut establish_connection();

    rust_db
        .run_pending_migrations(MIGRATIONS)
        .expect("failed running migrations");

    conns.push(rust_db);

    let go_db_url = env::var("GO_DB_URL").unwrap_or("".to_string());
    let mut go_db_conn: PgConnection;
    if go_db_url != "" {
        match PgConnection::establish(&go_db_url) {
            Ok(go_conn) => {
                go_db_conn = go_conn;
                conns.push(&mut go_db_conn);
            }
            Err(err) => {
                print!(
                    "could not connect to Go db even though env variable is set, err: {:?}",
                    err
                )
            }
        }
    } else {
        print!("Go DB URL not set");
    }

    seed_fake_data(conns);
}
