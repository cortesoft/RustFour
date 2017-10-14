//Ask a human
use std::io;
use super::Player;
use super::*;

pub struct Human {
    pub name: String
}

impl Player for Human {
    fn which_move(&self, board: &Board) -> u8 {
        println!("Play which column? (You are {}'s)", board.get_current_move());
        let mut my_move = String::new();
        let mut my_move_i: u8 = 0;
        loop {
            io::stdin().read_line(&mut my_move)
                .expect("Failed to read line");
            my_move_i = match my_move.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Sorry, {} is not a valid move", &my_move);
                    continue;
                }
            };
            if board.valid_move(my_move_i) {
                break;
            }
            println!("Sorry, {} is not a valid move", &my_move);
        }
        my_move_i
    }
}