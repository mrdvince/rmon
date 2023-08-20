use std::{env, io};

use rmon::repl;

fn main() {
    match env::var("USER") {
        Ok(user) => println!("Hello {user}! This is the..I have no idea what this is tbh!!"),
        Err(_) => println!("Hello !! This is the..I have no idea what this is tbh!!"),
    }
    println!("Feel to start typing");
    let input = io::stdin();
    let input_lock = input.lock();
    let output = io::stdout();
    repl::start(input_lock, output)
}
