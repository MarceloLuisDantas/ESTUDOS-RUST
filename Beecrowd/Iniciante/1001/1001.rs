fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Erro ao ler entrada");
    buffer
}

fn main() {
    let n1 = input().trim().parse::<i64>().unwrap();
    let n2 = input().trim().parse::<i64>().unwrap();
    println!("X = {0}", n1 + n2);
}