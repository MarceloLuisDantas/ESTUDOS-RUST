use std::io::*;
use std::str::FromStr;
use std::fmt::Debug;

fn input<T>() -> T where
    T: FromStr,
    T::Err: Debug,
{
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().parse::<T>().unwrap();
}

fn main() {
    let distancia: f32 = input();
    let litros: f32 = input();
    println!("{:.3} km/l", distancia / litros);
}