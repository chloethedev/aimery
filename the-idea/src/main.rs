// use rand::Rng;
// use std::cmp::Ordering;
use std::io;

fn main() {

    println!("\n...darkness...\n\n\n");

    
    println!("...warmth...\n\n");


    println!("name please?\n");

    let mut user_name = String::new();

    io::stdin()
        .read_line(&mut user_name)
        .expect("everlasting darkness relentlessly consumes...");

    println!("\nhello, {}, welcome to Everglass.", user_name.trim());

            
}

