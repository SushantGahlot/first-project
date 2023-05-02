use crate::DAO;
use async_trait::async_trait;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use std::env;

pub struct DB {
    pub pool: bb8::Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>,
}

#[async_trait]
impl DAO for DB {
    async fn new() -> DB {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(5)
            .build(config)
            .await
            .expect("error creating pool");

        DB { pool }
    }
}
