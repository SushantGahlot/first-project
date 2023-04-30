use crate::DAO;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use std::env;

pub struct DB {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl DAO for DB {
    fn new() -> DB {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: Pool<ConnectionManager<PgConnection>> = Pool::builder()
            .build(manager)
            .expect("unable to create db pool");
        DB { pool }
    }

    fn get_connection(
        &self,
    ) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
        self.pool.get()
    }
}
