//game module

mod board;
pub mod players;

use rand;
use rand::Rng;
use self::board::*;
use self::players::Player;
use self::players::human::Human;

pub fn play() {
    let player1 = Human {
        name: String::from("Daniel")
    };
    let mut board = Board::new_board(8, 8, 4);
    board.print();
    for i in 0..500 {
        println!("On turn {}", i);
        let mut mymove: u8 = 0;
        if let GamePiece::X = board.get_current_move() {
            mymove = player1.which_move(&board);
        } else {
            mymove = rand::thread_rng().gen_range(0, 8);
            while !board.valid_move(mymove){
                mymove = rand::thread_rng().gen_range(0, 8);
            }
        }
        println!("Playing {}", mymove);
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