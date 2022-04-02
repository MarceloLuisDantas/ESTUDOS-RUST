fn input() -> f32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Erro");
    buffer.trim().parse::<f32>().unwrap()
}

fn main() {
    let n1 = input() * 3.5;
    let n2 = input() * 7.5;    
    println!("MEDIA = {0:.5}", (n1 + n2) / 11.0);
}