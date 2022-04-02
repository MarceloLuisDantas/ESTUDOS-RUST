fn input() -> i64 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<i64>().unwrap()
}

fn main() {
    let numeros = vec![vec![input(), input()], 
                       vec![input(), input()]];

    let mut produtos = Vec::new();
    for lista in numeros {
        match lista.into_iter().reduce(|a, b| a * b) {
            Some(x) => produtos.push(x),
            None => panic!("Error ao fazer o produto")
        };
    }

    let dif = match produtos.into_iter().reduce(|a, b| a - b) {
        Some(x) => x,
        None => panic!("Error")
    };
 
    println!("DIFERENCA = {}", dif);
}