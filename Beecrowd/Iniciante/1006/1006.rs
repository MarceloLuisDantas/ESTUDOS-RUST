fn input() -> f32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<f32>().unwrap()
}

fn main() {
    let notas = [input() * 2.0, 
                 input() * 3.0, 
                 input() * 5.0];
                 
    // Funções que usam Generics em Rust precisa receber o tipo
    // que estão lidando pelo turbofish
    let media = notas.iter().sum::<f32>() / 10.0;
    println!("MEDIA = {0:.1}", media);
}
