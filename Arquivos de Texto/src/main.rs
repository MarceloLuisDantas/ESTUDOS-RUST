use std::io::{BufRead, BufReader};
mod arquivo;

struct Numeros {
    valores: Vec<i32>
}

impl Numeros {
    fn add_nun(&mut self, num: i32) {
        self.valores.push(num);
    }
    fn show_valores(&self) {
        for (i, valor) in self.valores.iter().enumerate() {
            println!("Numero {}º = {}", i + 1, valor);
        }
    }
}

fn main() {
    let input = arquivo::carrega_arquivo("./texto.txt");
    let buffered = BufReader::new(input);

    let mut numeros = Numeros { valores: Vec::new() };
    for line in buffered.lines() {
        match line {
            Ok(line) => {
                let num: i32 = line.parse::<i32>().unwrap();
                numeros.add_nun(num);
            },
            Err(err) => {
                panic!("Erro ao ler arquivos paos aberto | Errir {}", err)
            }
        }
    }
    numeros.show_valores();
}
