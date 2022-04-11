use std::io::stdin;

fn input() -> i32 {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}

fn main() {
    println!("{} minutos", input() * 2);
}