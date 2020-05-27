extern crate dotenv;

use diesel::prelude::SqliteConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub mod models;
pub mod schema;

pub fn build_conn() -> diesel::SqliteConnection {
    dotenv().ok();

    // set up database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
