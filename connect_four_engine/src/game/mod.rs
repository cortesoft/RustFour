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

    pub fn new_game(player_1: T, player_2: U, rows: u8, cols: u8, conn: u8) -> Game<T,U> {
        let board_template = Board::new_board(rows, cols, conn);
        Game {
            player_1,
            player_2,
            board_template
        }
    }

    pub fn play_game(&mut self, player_1_is_x: bool) -> u8 {
        let mut board = self.board_template.clone();
        let player_1_gamepiece = if player_1_is_x {
            GamePiece::X
        } else {
            GamePiece::O
        };
        board.print();
        for i in 0..500 {
            println!("On turn {}", i);
            let player_1_turn: bool; 
            let mymove = if player_1_gamepiece == board.get_current_move() {
                player_1_turn = true;
                self.player_1.which_move(&board)
            } else {
                player_1_turn = false;
                self.player_2.which_move(&board)
            };
            println!("Playing {}", mymove);
            board.play_piece(mymove);
            board.print();
            match board.have_winner() {
                1 => {
                    println!("X won!");
                    if player_1_turn {
                        self.player_1.win();
                        self.player_2.loss();
                    } else {
                        self.player_2.win();
                        self.player_1.loss();
                    }
                    return 1;
                },
                2 => {
                    println!("O won!");
                    if player_1_turn {
                        self.player_1.win();
                        self.player_2.loss();
                    } else {
                        self.player_2.win();
                        self.player_1.loss();
                    }
                    return 2;
                },
                _ => ()
            }
            if !board.any_legal_moves() {
                print!("There are no legal moves left! We have a draw!");
                return 0;
            }
        }
        0
    }
}
