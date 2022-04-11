use std::io::stdin;

fn input() -> i32 {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}

fn main() {
    println!("{:.3}", (input() * input()) as f32 / 12.0);
}