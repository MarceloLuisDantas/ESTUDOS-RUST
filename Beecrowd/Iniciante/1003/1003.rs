fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Erro ao ler entrada");
    buffer
}

fn main() {
    let nun1 = input().trim().parse::<i64>().unwrap();
    let nun2 = input().trim().parse::<i64>().unwrap();
    let soma = nun1 + nun2;
    println!("SOMA = {0}", soma);
}