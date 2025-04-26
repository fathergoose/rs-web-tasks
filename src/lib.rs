use self::models::{NewThing, Thing};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_thing(conn: &mut PgConnection, title: &str, details: &str) -> Thing {
    // What's the value of having this use statement nested inside this function?
    // It appears to work fine if the line below is placed atop this file too
    use crate::schema::things;

    let new_thing = NewThing { title, details };

    diesel::insert_into(things::table)
        .values(&new_thing)
        .returning(Thing::as_returning())
        .get_result(conn)
        .expect("Error saving enw post")
}
