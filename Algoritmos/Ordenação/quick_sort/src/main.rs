/*

    Contribuidores
        - Dromedario de chapeu
    
    O Quick Sort, é um dos algoritmos de ordenação padrões de se estudar. 
    Ele é famoso por ser relativamente eficiente na maioria dos casos, tendo
    complexidade O(n log n). Porem em casos especificos ele pode chegar a ser
    O(n²) nos piores casos, como a lista estar basicamente invertida por exemplo.
    O Quick Sort adota a estrategia de "dividir e conquistar", que é resumidamente
    dividir a lista em sub 2 listas e organizar essas sub listas. A cada execução
    é escolhido o Pivot, e a partir deste Pivot nos comparamos todos os elementos
    da lista ate encontrar algum valor menor ou igual ao pivot, quando encontrarmos 
    um este valor, ele é colocado a frente do pivot. Quando pasarmos por toda a 
    lista, nos fazemos o swap entre o pivot e o ultimo indice que foi coloca a sua 
    frene, assim nos teremos a garantia de que todos os itens a esquerda do pivot 
    seram menores, e os a direita maiores. E assim nos pegamos essas 2 novas listas, 
    e fazemos o mesmo processo.

    Recomendo dar uma olhada mais profunda sobre o funcionado e casos especificos,
    como os diferentes metodos de partição e funcionado com multiplos pivot's.
        Links: https://pt.wikipedia.org/wiki/Quicksort
               https://rosettacode.org/wiki/Sorting_algorithms/Quicksort

*/

use rand::{thread_rng, Rng};

fn partition(lista: &mut [i32], l: usize, r: usize) -> isize {
    let pivot = lista[l];
    let mut count = l;
    for j in l + 1..r + 1 {
        if lista[j] <= pivot {
            count += 1;
            lista.swap(count, j);
        }
    }
    lista.swap(l, count);
    return count as isize;
}

fn quick_sort(lista: &mut [i32],  l: isize, r: isize) {
    if l < r {
        let indice_pivot = partition(lista, l as usize, r as usize);
        quick_sort(lista, l, indice_pivot - 1);
        quick_sort(lista, indice_pivot + 1, r);
    }   
}

fn gera_lista(len: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut lista = Vec::<i32>::new();
    for _ in 0..len {
        let nun = rng.gen_range(0..10);
        lista.push(nun);
    }
    return lista;
}

fn main() {
    let mut lista = vec![6,21,7,12,34,65,23,74,4,52];
    // let mut lista = gera_lista(10);
    let len = lista.len() as isize;

    for i in lista.iter() {
        print!("{} ", *i);
    }
    println!();
    
    quick_sort(&mut lista, 0, len - 1);

    for i in lista.iter() {
        print!("{} ", *i);
    }
}

// #[cfg(test)]
// mod test {
//     use crate::quick_sort;

//     #[test]
//     fn quick() {
//         let lista = vec![9, 8, 7, 6, 5 ,4 ,3 ,2 ,1];
//         assert_eq!(vec![1,2,3,4,5,6,7,8,9], quick_sort(lista.clone()));
//     }
// }