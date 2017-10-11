mod board;

use std::io;
use rand;
use rand::Rng;

pub fn play() {
    let mut board = board::new_board(8, 8);
    for i in 0..20 {
        println!("On turn {}", i);
        board.print();
        let mymove = rand::thread_rng().gen_range(0, 8);
        board.play_piece(mymove);
    }
    board.print();
}