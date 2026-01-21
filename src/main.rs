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
    else if s.split_whitespace().nth(0) == Some("pwd"){
        println!("{}", env::current_dir().unwrap().display());
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
