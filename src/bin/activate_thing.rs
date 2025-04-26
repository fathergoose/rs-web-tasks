use self::models::Thing;
use diesel::prelude::*;
use rs_web_tasks::*;
use std::env::args;

fn main() {
    use self::schema::things::dsl::{active, things};

    let id = args()
        .nth(1)
        .expect("activate thing requires id")
        .parse::<i32>()
        .expect("invalid id");

    let connection = &mut establish_connection();

    let thing: Thing = diesel::update(things.find(id))
        .set(active.eq(true))
        .returning(Thing::as_returning())
        .get_result(connection)
        .unwrap();

    println!("Activated thing {}", thing.title);
}
