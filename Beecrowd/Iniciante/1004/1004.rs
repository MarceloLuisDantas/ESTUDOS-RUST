fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Erro ao ler entrada");
    buffer
}

fn main() {
    let nun1 = input().trim().parse::<i32>().unwrap();
    let nun2 = input().trim().parse::<i32>().unwrap();
    println!("PROD = {0}", nun1 * nun2);
}