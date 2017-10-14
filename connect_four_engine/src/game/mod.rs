//game module

mod board;
pub mod players;

use self::board::*;
use self::players::Player;
use self::players::human::Human;
use self::players::random::Random;

pub fn play() {
    let player1 = Human {
        name: String::from("Daniel")
    };
    let player2 = Random {};
    let mut board = Board::new_board(8, 8, 4);
    board.print();
    for i in 0..500 {
        println!("On turn {}", i);
        let mymove: u8;
        if let GamePiece::X = board.get_current_move() {
            mymove = player1.which_move(&board);
        } else {
            mymove = player2.which_move(&board);
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