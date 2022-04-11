use std::io::stdin;

fn input() -> i32 {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}

fn print_notas(mut valor: i32) {
    println!("{}", valor);
    for nota in vec![100, 50, 20, 10, 5, 2, 1].iter() {
        println!("{} nota(s) de R$ {},00", valor / nota, nota);
        valor = valor % nota;
    }
}

fn main() {
    print_notas(input());
}