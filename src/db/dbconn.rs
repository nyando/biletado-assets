use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;

use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// lazily initialize the DB connection pool
// and make it available through the reference POOL
lazy_static! {
    static ref POOL : Pool = {

        let db_user = env::var("POSTGRES_ASSETS_USER")
            .expect("POSTGRES_ASSETS_USER environment variable not set");
        let db_pass = env::var("POSTGRES_ASSETS_PASSWORD")
            .expect("POSTGRES_ASSETS_PASSWORD environment variable not set");
        let db_name = env::var("POSTGRES_ASSETS_DBNAME")
            .expect("POSTGRES_ASSETS_DBNAME environment variable not set");
        let db_host = env::var("POSTGRES_ASSETS_HOST")
            .expect("POSTGRES_ASSETS_HOST environment variable not set");
        let db_port = env::var("POSTGRES_ASSETS_PORT")
            .expect("POSTGRES_ASSETS_PORT environment variable not set");

        let db_url = format!("postgres://{}:{}@{}:{}/{}", db_user, db_pass, db_host, db_port, db_name);

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("failed to create db pool")
    };
}

/// Initialize the DB connection pool.
pub fn init() -> Result<DbConnection, r2d2::Error> {
    lazy_static::initialize(&POOL);
    connection()
}

/// Get a database connection from the pool.
pub fn connection() -> Result<DbConnection, r2d2::Error> {
    POOL.get()
}