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
    let bb = s.trim().split_whitespace().next() == Some("echo");
    if s.trim().is_empty(){continue};
    if bb {
        let ans = s.strip_prefix("echo").unwrap().trim();
        println!("{}",ans);
        continue;
    }
    println!("{}: command not found",s.trim());
    }


    
}
