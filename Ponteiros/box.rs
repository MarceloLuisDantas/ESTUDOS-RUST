/*
    Box é um tipo de smart pointer que deve ser utilizado quando lidamos com valores que n tem um tamanho fixo, como Enum ou  Estruturas, quando se tem uma grande quantidade de dados para serem transferidos sem serem copiados ou quando o que importa é se o tipo a ser copiado implemente uma trait especifica  

    Box permite a criação de tipos recursivos, quando um tipo possui ela mesma. 
*/

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}