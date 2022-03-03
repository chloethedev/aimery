use rand::Rng;
use std::cmp::Ordering;
use std::io;
use proceed::any_or_quit_with;

fn pause() {
      if !any_or_quit_with('q') {
        println!("Quitting.");
        return;
    }}

fn main() {

    println!("\n...darkness...\n");

    pause();

    println!("\n...warmth...\n");

    pause();
    
    println!("\n...tell us your name please?\n");

    let mut user_name = String::new();

   io:: stdin()
        .read_line(&mut user_name)
        .expect("everlasting darkness relentlessly consumes...");

    println!("\nhello, {}, welcome to Everglass.\n", user_name.trim());

    pause();

    println!("enveloped in darkness...a flicker... hope?");

    pause();
    
    println!("crawling, walking, tumbling, fubling, running, stop!");
    
    pause();
    
    println!("scared, nervous, shaking, crying, stumbling, crumbling, stop!");
    
    pause();
    
    println!("...a flicker of light?... hope?!");
    
    pause();
    
    println!("crawling, walking, running, tumbling, stumbling...");
    
    pause();
    
    let absurd_num = rand::thread_rng().gen_range(1..20);

    loop {
        println!("...please input positive integer to generate absurdity");

        let mut input = String::new();

        io:: stdin()
            .read_line(&mut input)
            .expect("everlasting darkness relentlessly consumes...");

         let input: u32 = match input.trim().parse() {
             Ok(num) => num,
             Err(_) => continue,
         };

         println!(".....");

         match input.cmp(&absurd_num) {
            Ordering::Less => println!("not enough absurdity"),
            Ordering::Greater => println!("i think that is too much"),
            Ordering::Equal => {
                println!("\nthe flame is now bright enough that you can just make out your hands as they reach out infront of you...");
                break;
            }

         }
    }       
}

