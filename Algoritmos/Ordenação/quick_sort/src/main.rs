use rand::{thread_rng, Rng};

fn partition(lista: &mut [i32], l: usize, r: usize) -> isize{
    let pivot = lista[r];
    let mut count = 0;
    for i in l + 1..r {
        if pivot <= lista[i] {
            count += 1;
            lista.swap(count, i);
        }
    }
    lista.swap(r, count);
    return count as isize;
}

fn quick_sort(lista: &mut [i32],  l: isize, r: isize) {
    if l < r {
        println!("{}, {}", l, r);
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
    // let lista = gera_lista(10);
    let mut lista = gera_lista(10);
    let len = (lista.len() - 1) as isize;

    for i in lista.iter() {
        print!("{} ", *i);
    }
    println!();
    
    quick_sort(&mut lista, 0, len);

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