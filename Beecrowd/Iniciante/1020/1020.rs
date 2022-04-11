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
    let mut dias: i32 = input();
    let mut anos = 0;
    let mut meses = 0;

    if dias >= 360 {
        anos = dias / 360;
        dias -= anos * 360;    
    }
    if dias >= 30 {
        meses = dias / 30;
        dias -= meses * 30; 
    }
    println!("{} ano(s)", anos);
    println!("{} mes(es)", meses);
    println!("{} dia(s)", dias);
}