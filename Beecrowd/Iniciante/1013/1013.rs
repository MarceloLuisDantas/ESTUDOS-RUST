use std::io::*;

fn input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn maior(a: i32, b: i32) -> i32 {
    return (a + b + (a - b).abs()) / 2
}

fn main() {
    let buffer = input();
    let valores = buffer.split(" ")
                        .filter_map(|x| x.parse::<i32>().ok())
                        .collect::<Vec<i32>>();
    let maior = maior(maior(valores[0], valores[1]), valores[2]);
    println!("{} eh o maior", maior);
}