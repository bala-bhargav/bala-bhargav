#[allow(unused_imports)]
use std::io::{self, Write};
use pathsearch::find_executable_in_path;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    let bytes = io::stdin().read_line(&mut s).unwrap();
    let start_type = s.trim().split_whitespace().next() == Some("type");
    let start_echo = s.trim().split_whitespace().next() == Some("echo");
    let start_exit = s.trim().split_whitespace().next() == Some("exit"); 
    if s.trim().is_empty(){continue};
    if start_type {
        if s.split_whitespace().nth(1) == Some("echo") {
        println!("{}","echo is a shell builtin");
        }
        else if s.split_whitespace().nth(1) == Some("exit") { 
        println!("{}","exit is a shell builtin");
        }
        else if s.split_whitespace().nth(1) == Some("type") { 
        println!("{}","type is a shell builtin");
        }
        else if let Some(path) = find_executable_in_path(s.split_whitespace().nth(1)){
            println!("{} is {}",s.split_whitespace().nth(1).unwrap(),path.display());
        }
        else {
         let rest = s
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join(" ");
         println!("{}: not found",rest.trim()); 
        }
    }
    else if start_echo {
        let rest = s
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join(" ");
        println!("{}",rest.trim()); 
    }
    else if start_exit {
        break;
    }
    else{ 
        println!("{}: command not found",s.trim());
                    
    }
    
    }


    
}
