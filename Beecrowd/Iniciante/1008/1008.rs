fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn main() {
    let numero = input().trim().parse::<i64>().unwrap();
    let horas = input().trim().parse::<i64>().unwrap();
    let salario = input().trim().parse::<f32>().unwrap() * (horas as f32);
    println!("NUMBER = {}", numero);
    println!("SALARY = U$ {:.2}", salario);
}