use std::env;

use diesel::{Connection, PgConnection};

pub fn open_connection() -> PgConnection {
    let dsn = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&dsn)
        .unwrap_or_else(|_| panic!("Failed to connect to PostgreSQL Server"))
}
