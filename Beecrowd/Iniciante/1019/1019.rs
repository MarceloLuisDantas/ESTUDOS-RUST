use std::io::*;
use std::str::FromStr;
use std::fmt::Debug;

fn input<T>() -> T where
    T: FromStr,
    T::Err: Debug,
{
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}

fn main() {
    let mut segundos: i32 = input();
    let mut minutos = segundos / 60;
    let horas = minutos / 60;
    minutos = minutos % 60;
    segundos = (segundos % 60) % 60;
    println!("{}:{}:{}", horas, minutos, segundos);
}