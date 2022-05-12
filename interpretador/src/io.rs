use std::io::{stdin, stdout, Write};

pub fn input() -> String {
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
    return buffer.trim().to_string();
}

pub fn print(label: &str) {
    print!("{}", label);
    let _ = stdout().flush();
}

pub fn clear_screen() { 
    std::process::Command::new("clear").status().unwrap(); 
}
