mod io;
use eval::eval;
use io::{clear_screen, input, print};
mod memoria;
use memoria::Memoria;
mod eval;

fn main() {
    let mut memoria = Memoria::new();
    clear_screen();
    loop {
        print("|> ");
        let linha = input();
        match eval(&linha, &mut memoria) {
            Ok(v) => print(&format!("{}", v)),
            Err(msg) => {
                if msg == "cls".to_string() {
                    clear_screen();
                } else {
                    print(&msg);
                }
            }
        }
        print("\n");
    }
}
