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

fn print_notas(mut valor: i32) -> i32 {
    println!("NOTAS:");
    for nota in vec![100, 50, 20, 10, 5, 2].iter() {
        println!("{} nota(s) de R$ {},00", valor / nota, nota);
        valor = valor % nota;
    }
    return valor * 100;
}

fn print_moedas(mut valor: i32) {
    println!("MOEDAS:");
    for moeda in vec![1.0, 0.50, 0.25, 0.10, 0.05, 0.01].iter() {
        println!("{} MOEDAS(s) de R$ {:.2}", valor / (moeda * 100.0) as i32, moeda);
        valor = valor % (moeda * 100.0) as i32;
    }
}

fn div(valor: f32) -> (i32, i32) {
    let reais = valor as i32;
    let centavos = (valor * 100.0) as i32 - reais * 100;
    return (reais, centavos); 
}

fn main() {    
    let (r, mut c) = div(input());
    c += print_notas(r);
    print_moedas(c);
}




