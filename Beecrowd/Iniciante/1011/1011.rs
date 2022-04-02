fn input() -> f32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Erro ao ler entrada");
    buffer.trim().parse::<f32>().expect("Erro ao converter tipo")
}

fn main() {
    let area = ((4.0/3.0) * 3.14159) * (input().powf(3.0)); 
    let _x = (area * 1000.0).round() / 1000.0;
    println!("VOLUME - A2 = {:.4}", area);
    println!("VOLUME - A1 = {:.3}", (4.0 / 3.0) * 3.14159 * 15.0_f32.powf(3.0));
    //println!("VOLUME - X = {:.3}", x);
}