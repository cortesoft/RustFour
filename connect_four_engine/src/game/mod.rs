mod board;

use std::io;
use rand;
use rand::Rng;

pub fn play() {
    let mut board = board::new_board(8, 8, 4);
    board.print();
    for i in 0..200 {
        println!("On turn {}", i);
        let mut mymove = rand::thread_rng().gen_range(0, 8);
        while !board.valid_move(mymove){
            mymove = rand::thread_rng().gen_range(0, 8);
        }
        board.play_piece(mymove);
        board.print();
        match board.have_winner() {
            1 => {
                println!("X won!");
                break;
            },
            2 => {
                println!("O won!");
                break;
            },
            _ => ()
        }
    }
    
}