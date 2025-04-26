use rs_web_tasks::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut details = String::new();

    println!("Title for new thing:");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("\nDetails:");
    println!("\nComplete details with {EOF}:");
    stdin().read_to_string(&mut details).unwrap();

    let thing = create_thing(connection, title, &details);
    println!("\nSaved new thing {title} with id: {}", thing.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
