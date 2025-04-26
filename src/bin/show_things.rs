use crate::models::*;
use diesel::prelude::*;
use rs_web_tasks::*;

fn main() {
    use self::schema::things::dsl::*;

    let connection = &mut establish_connection();
    let results: Vec<Thing> = things
        .filter(active.eq(true))
        .limit(5)
        .select(Thing::as_select())
        .load(connection)
        .expect("Error loading things");

    println!("Displaying {} things", results.len());
    for thing in results {
        println!("{}", thing.title);
        println!("------------------\n");
        if let Some(d) = thing.details {
            println!("{}", d)
        }
    }
}
