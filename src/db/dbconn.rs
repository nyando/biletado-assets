use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;

use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL : Pool = {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    connection().expect("failed to connect to DB");
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}

pub fn connection() -> Result<DbConnection, r2d2::Error> {
    POOL.get() // TODO handle error case
}