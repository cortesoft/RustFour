//game module

mod board;
pub mod players;

use self::board::*;
use self::players::Player;

pub struct Game<T: Player, U: Player> {
    pub player_1: T,
    pub player_2: U,
    board_template: Board
}

impl<T: Player, U: Player> Game<T, U> {

    pub fn new_game(mut player_1: T, mut player_2: U, rows: u8, cols: u8, conn: u8) -> Game<T,U> {
        let mut board_template = Board::new_board(rows, cols, conn);
        Game {
            player_1,
            player_2,
            board_template
        }
    }

    pub fn play_game(&mut self) -> u8 {
        let mut board = self.board_template.clone();
        board.print();
        for i in 0..500 {
            println!("On turn {}", i);
            let mymove = if GamePiece::X == board.get_current_move() {
                self.player_1.which_move(&board)
            } else {
                self.player_2.which_move(&board)
            };
            println!("Playing {}", mymove);
            board.play_piece(mymove);
            board.print();
            match board.have_winner() {
                1 => {
                    println!("X won!");
                    return 1;
                },
                2 => {
                    println!("O won!");
                    return 2;
                },
                _ => ()
            }
        }
        0
    }
}
