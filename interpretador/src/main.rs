mod testes;
mod sintaxe;
mod io;
mod memoria;
mod eval;
mod erros;

use io::{clear_screen, input, print};
use memoria::Memoria;
use eval::{eval, Resultado};

fn main() {
    let mut memoria = Memoria::new();
    clear_screen();
    loop {
        print("|> ");
        let linha = input();
        match eval(&linha, &mut memoria) {
            Ok(r) => {
                match r {
                    Resultado::Msg { r: msg } => { 
                        if msg == "cls".to_string() {
                            clear_screen()
                        } else {
                            print(&format!("{}", msg)) 
                        }
                    }
                    Resultado::Num  { r: num } => { print(&format!("{}", num)) }
                    Resultado::Valor { r: tipo } => { print(&format!("{}", tipo)) }
                }
            },
            Err(msg) => {
                print(&msg)
            }
        }
        print("\n");
    }
}
