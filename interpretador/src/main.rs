mod sintaxe;
mod io;
mod memoria;
mod eval;

use io::{clear_screen, input, print};
use memoria::Memoria;
use eval::{eval};

fn main() {
    let mut memoria = Memoria::new();
    clear_screen();
    loop {
        print("|> ");
        let linha = input();
        match eval(&linha, &mut memoria) {
            Ok(v) => {
                if &v.valor() == "cls" {
                    clear_screen();
                } else {
                    print(&format!("{}", v.valor()))
                }
            },
            Err(msg) => {
                print(&msg);   
            }
        }
        print("\n");
    }
}
