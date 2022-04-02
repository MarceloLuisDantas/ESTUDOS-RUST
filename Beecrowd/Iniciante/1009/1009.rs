fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn main() {
    let _ = input();
    let fixo = input().trim().parse::<f32>().unwrap();
    let vendas = input().trim().parse::<f32>().unwrap();
    let total = ((fixo + (vendas * 15.0 / 100.0)) * 100.0).round() / 100.0;
    println!("TOTAL = R$ {:.2}", total);
}