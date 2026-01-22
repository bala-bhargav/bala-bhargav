#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::env;
use std::fs;
use std::path::Path;
 use std::process::Command;

fn find_executable_in_path(cmd: &str) -> Option<String> {
    if cmd.contains("/") {
        let p = Path::new(cmd);
        if p.exists() && fs::metadata(p).map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false) {
            return Some(cmd.to_string());
        } else {
            return None;
        }
    }

    if let Ok(paths) = env::var("PATH") {
        for path in paths.split(':') {
            let full_path = Path::new(path).join(cmd);
            if full_path.exists() {
                if let Ok(metadata) = fs::metadata(&full_path) {
                    if metadata.permissions().mode() & 0o111 != 0 {
                        return Some(full_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    None
}

 fn parse_args(input: &str)->Vec<String>{
    let mut args = Vec::new();
    let mut curr = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut chars = input.chars().peekable();


    while let Some(c) = chars.next(){
         if c == '\'' && !in_double {
            in_single = !in_single;
        } else if c == '"' && !in_single {
            in_double = !in_double;
        }
        else if (c == ' ' || c == '\t') && !in_single && !in_double{
            if !curr.is_empty(){
                args.push(curr.clone());
                curr = String::new();
            }
            while let Some(next) = chars.peek() {
                if *next == ' ' || *next == '\t' {
                    chars.next();
                } else {
                    break;
                }
            }
        }
        else{
            curr.push(c);
        }
        
    }
    if !curr.is_empty(){
        args.push(curr);
    }
    args

 }

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
        else if s.split_whitespace().nth(1) == Some("pwd") {
         println!("pwd is a shell builtin");
        }
        else if let Some(cmd) = s.split_whitespace().nth(1){
              if let Some(path) = find_executable_in_path(cmd) { 
                    println!("{} is {}", cmd, path);                
                } else {                                            
                    println!("{}: not found", cmd);                 
                } 
        }
        else {
        println!("type: missing argument"); 
        }
    }
    else if start_echo {
     let args = parse_args(s.trim());
    let out = args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
     let mut chars = out.chars().peekable();
     let mut curr = String::new();

     while let Some(c) = chars.next(){
        if c == '\''{
            let Some(nxt_char) = chars.peek();
            curr.push(nxt_char);
            chars.next();
        }
        else{
            curr.push(c);
        }
     }
     println!("{}",curr);
    }
    else if s.split_whitespace().nth(0) == Some("cat"){
        let args = parse_args(s.trim());
        let cmd = &args[0];
        let arg = &args[1..];
        Command::new(cmd)
            .args(arg)
            .status();

    }
    else if start_exit {
        break;
    }
    else if s.split_whitespace().nth(0) == Some("pwd"){
        println!("{}", env::current_dir().unwrap().display());
    }
    else if s.split_whitespace().nth(0) == Some("cd"){
         if let Some(bb) = s.split_whitespace().nth(1){ 
            if bb == "~"{
                let home = env::var("HOME").unwrap();
                let _ = env::set_current_dir(home);                 
            }
            else if let Ok(path) = env::set_current_dir(bb){                 
            }
            else{
                println!("cd: {}: No such file or directory",bb);
            }
         }
    }
    else{ 
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        if parts.is_empty(){continue};
        let cmd = parts[0]; 
        if let Some(path) = find_executable_in_path(cmd) {
            Command::new(cmd)
                 .args(&parts[1..])
                 .status();
        }
        else {
        println!("{}: command not found",cmd);
        }    

    }
    }
}
