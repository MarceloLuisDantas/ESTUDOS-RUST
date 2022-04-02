fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Erro ao ler entrada");
    buffer
}

fn main() {
    let raio = input().trim().parse::<f64>().unwrap();
    println!("A={0:.4}", 3.14159 * (raio.powf(2.0)))
}