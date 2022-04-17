/*
    Contribuidores
        - Dromedario de Chapéu

    Fatorial é uma função matematica que consistem em realizar
	a multiplicação de todos os antecessores de um numero.

	Ex: 5! = 5 * 4 * 3 * 2 * 1 = 120 
*/

// A diferença desta implementação para a com recursão é 
// que nesta versão inves de dar o retorno por recursão, é 
// utilizado um for que percorre uma lista que vai de 0 a valor + 1
fn fatorial(valor: usize) -> usize {
    let mut total = 1;
    for i in (0..valor + 1).collect::<Vec<usize>>().iter() {
        if *i != 0 {
            total *= *i;
        }
    }
    return total
}

fn main() {
    println!("{}", fatorial(10));
}

#[cfg(test)]
mod test {
    use super::*;
    fn teste_fatorial() {
        assert_eq!(fatorial(0), 1);
        assert_eq!(fatorial(1), 1);
        assert_eq!(fatorial(10), 3628800);
    }
}