#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    while(true){
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    let bytes = io::stdin().read_line(&mut s).unwrap();
    if(bytes == 0)break;
    if( s.trim().is_empty())continue;
    println!("{}: command not found",s.trim());
    }


    
}
