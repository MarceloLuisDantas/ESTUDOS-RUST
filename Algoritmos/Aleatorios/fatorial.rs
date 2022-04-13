fn fatorial(valor: usize) -> usize {
    if valor == 1 || valor == 0{
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