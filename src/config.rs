use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod schema;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    println!("Connected to the database!");
}
