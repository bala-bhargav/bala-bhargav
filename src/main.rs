#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    let bytes = io::stdin().read_line(&mut s).unwrap();
    if s.trim() == "exit" {break};
    if s.trim().is_empty(){continue};
    println!("{}: command not found",s.trim());
    }


    
}
