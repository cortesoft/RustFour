//Just return smart moves
use rand;
use rand::Rng;
use super::Player;
use super::*;

#[derive(PartialEq)]
enum MoveValue {
    MaybeWin(isize),
    AlwaysWin,
    AlwaysLose,
    InvalidMove
}

use self::MoveValue::*;

pub struct Robot {
    max_depth: u8,
    val_per_connection: u16,
    my_game_piece: GamePiece
}

impl Player for Robot {
    fn which_move(&mut self, board: &Board) -> u8 {
        self.my_game_piece = board.get_current_move();
        self.val_per_connection = 1000 / (3 * (board.connect_number as u16 - 1) - 1);
        let mut mymove = u8::max_value();
        let mut max_value: isize = isize::min_value();
        let mut possible_moves = Vec::new();
        for col in 0..board.num_rows {
            match self.value_for_move(col, &board, 0) {
                AlwaysWin => { return col; },
                MaybeWin(move_val) => {
                    if move_val > max_value {
                        max_value = move_val;
                        mymove = col;
                    } else if move_val == max_value {
                        possible_moves.push(col);
                    }
                },
                _ => ()
            }
        }
        if possible_moves.len() > 0 {
            possible_moves.push(mymove);
            let move_choice: usize = rand::thread_rng().gen_range(0, possible_moves.len());
            mymove = possible_moves[move_choice];
        }
        while !board.valid_move(mymove){
            mymove = rand::thread_rng().gen_range(0, board.num_rows);
        }
        mymove
    }
}

impl Robot {
    pub fn build_robot(max_depth: u8) -> Robot {
        Robot {
            max_depth,
            my_game_piece: GamePiece::Dash,
            val_per_connection: 0
        }
    }

    fn value_for_move(&self, the_move: u8, board: &Board, current_depth: u8) -> MoveValue {
        if !board.valid_move(the_move) {
            return InvalidMove;
        }
        let is_my_move = self.my_game_piece == board.get_current_move();
        let mut board_cp = board.clone();
        board_cp.play_piece(the_move);
        if board_cp.have_winner() > 0 {
            if is_my_move {
                return AlwaysWin;
            } else {
                return AlwaysLose;
            }
        }
        if current_depth >= self.max_depth {
            return MaybeWin(self.value_for_streak(&board_cp, is_my_move));
        }
        let mut valid_move_count: u8 = 0;
        let mut win_count: u8 = 0;
        let mut loss_count: u8 = 0;
        let mut value_sum: isize = 0;
        for col in 0..board_cp.num_rows {
            match self.value_for_move(col, &board_cp, current_depth + 1) {
                AlwaysWin => {
                    if is_my_move {
                        //If every move the computer makes leads
                        //to me winning, yay!
                        win_count += 1;
                        valid_move_count += 1;
                        value_sum += 1500;
                    } else {
                        //I have a winning move to follow the comp
                        return AlwaysWin;
                    }
                },
                AlwaysLose => {
                    if is_my_move {
                        //I will lose if i make the root play
                        return AlwaysLose;
                    } else {
                        loss_count += 1;
                        valid_move_count += 1;
                        value_sum -= 1500;
                    }
                },
                MaybeWin(move_val) => {
                    value_sum += move_val;
                    valid_move_count += 1;
                },
                _ => ()
            }
        }
        //Ok did not lead to victory or defeat automatically
        if win_count == valid_move_count {
            return AlwaysWin;
        }
        if loss_count == valid_move_count {
            return AlwaysLose;
        }
        let future_val = value_sum / valid_move_count as isize;
        MaybeWin((future_val + self.value_for_streak(&board_cp, is_my_move)) / 2)
    }

    fn value_for_streak(&self, board: &Board, is_my_move: bool) -> isize {
        let tc_val = board.total_connected() as u16 * self.val_per_connection;
        if is_my_move {
            tc_val as isize
        } else {
            0 - (tc_val as isize)
        }
    }
}