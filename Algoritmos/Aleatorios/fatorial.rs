/*
    Contribuidores
		- Heitor582 
        - Dromedario de Chapéu

    Fatorial é uma função matematica que consistem em realizar
	a multiplicação de todos os antecessores de um numero.

	Ex: 5! = 5 * 4 * 3 * 2 * 1 = 120 
*/

// Para realizar uma fatoração com recursão basta fazer o retorno
// de uma função ser valor * a propia função recebendo valor - 1 
fn fatorial(valor: usize) -> usize {
	// Para que não chege a multiplicar por 0 quando chegamos a 1 ou 0
	// é retornado 1 para que o utlimo valor sejá multilpicado por 1
    if valor <= 1 {
        return 1;
    }
    return valor * (fatorial(valor - 1));
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