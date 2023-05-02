use crate::DAO;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use std::env;
use async_trait::async_trait;

pub struct DB {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait]
impl DAO for DB {
    async fn new() -> DB {
        env::set_var("DATABASE_URL", "postgres://rust:password@localhost:5432/rustdb");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: Pool<ConnectionManager<PgConnection>> = Pool::builder()
            .build(manager)
            .expect("unable to create db pool");
        DB { pool }
    }

    async fn get_connection(
        &self,
    ) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
        self.pool.get()
    }
}
