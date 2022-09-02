use std::{thread, time};
use ncurses::*;
mod game;

fn setup() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn main() {   
    setup();
    let sleep_time = time::Duration::from_millis(50);
    let mut board = game::cria_board(65, 132);
    let mut step_by_step = false;
    let mut quit = false;
    while !quit {
        for (linha, conteudo) in board.board.iter().enumerate() {
            let mut coluna = 0;
            for (_, celula) in conteudo.iter().enumerate() {
                mv(linha as i32, coluna as i32);
                match *celula {
                    '*' => _ = addstr("  "),
                    '#' => _ = addstr("##"),
                    '-' => _ = addstr("~~"),
                    'l' => _ = addstr(" |"),
                    'r' => _ = addstr("| "),
                    _ => {  },
                }
                coluna += 2;
            }
        }

        timeout(1);
        let key = getch();
        match key as u8 as char {
            'p' => step_by_step = !step_by_step,
            'q' => quit = true,
            'n' => board.next_generation(vec![3], vec![2, 3]),
            _ => { }
        }
        if !step_by_step {
            board.next_generation(vec![3], vec![2, 3]);
            thread::sleep(sleep_time);
        }
        refresh();
    }
    endwin();
}
