use std::str::FromStr;
use std::fmt::Debug;
use std::io::stdin;

fn input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn linha_to_vec<T>(linha: String) -> Vec<T> where
    T: FromStr,
    T::Err: Debug,
{
    linha.split(" ")
         .filter_map(|x| x.parse::<T>().ok())
         .collect::<Vec<T>>()
}

fn distancia(p1: Vec<f32>, p2: Vec<f32>) -> f32 {
    ((p1[0] - p2[0]).powf(2.0) + (p1[1] - p2[1]).powf(2.0)).sqrt()
}

fn main() {
    let valores1 = linha_to_vec(input());
    let valores2 = linha_to_vec(input());

    let distancia = distancia(valores1, valores2);
    println!("{:.4}", distancia);
}