//Just return smarter moves
use std::io;
use rand;
use rand::Rng;
use scoped_threadpool::Pool;
//use std::sync::Arc;
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

pub struct SmartBot {
    max_depth: u8,
    val_per_connection: u16,
    my_game_piece: GamePiece
}

struct MoveValueFor {
    move_val: MoveValue,
    column: u8
}

impl Player for SmartBot {
    fn which_move(&mut self, board: &Board) -> u8 {
        self.my_game_piece = board.get_current_move();
        self.val_per_connection = 1000 / (3 * (board.connect_number as u16 - 1) - 1);
        let mut mymove = u8::max_value();
        let mut max_value: isize = isize::min_value();
        let mut possible_moves = Vec::new();
        let mut move_vals: Vec<MoveValueFor> = Vec::new();
        for col in 0..board.num_columns {
            move_vals.push(MoveValueFor { move_val: InvalidMove, column: col});
        }
        let mut thread_pool = Pool::new(board.num_columns as u32);
        println!("Starting threads");
        thread_pool.scoped(|scope| {
            for mvf in &mut move_vals {
                let my_self = &self;
                scope.execute(move || {
                    println!("In thread, checking {}", mvf.column);
                    let this_move_val = my_self.value_for_move(mvf.column, &board, 0);
                    println!("Got value for {} in thread. Storing...", mvf.column);
                    mvf.move_val = this_move_val;
                });
            }
        });
        println!("Thread are done, checking values");
        for move_val_for in move_vals.iter() {
            match move_val_for.move_val {
                AlwaysWin => {
                    println!("Got an AlwaysWin for move {}", move_val_for.column);
                    return move_val_for.column; 
                },
                MaybeWin(move_val) => {
                    println!("Got MaybeWin with val {} for {}",
                        move_val, move_val_for.column);
                    if move_val > max_value {
                        println!("{} is now the max value", move_val_for.column);
                        possible_moves = Vec::new();
                        max_value = move_val;
                        mymove = move_val_for.column;
                    } else if move_val == max_value {
                        println!("{} has the same move_val, adding to possibles",
                            move_val_for.column);
                        possible_moves.push(move_val_for.column);
                    }
                },
                AlwaysLose => println!("Got AlwaysLose for {}", move_val_for.column),
                InvalidMove => println!("Got invalid move for {}", move_val_for.column)
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

    fn build_player() -> SmartBot {
        let mut max_depth: u8;
        let mut choice = String::new();
        loop {
            println!("For SmartBot, how deep to look?");
            io::stdin().read_line(&mut choice)
                .expect("Failed to read line");
            max_depth = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Sorry, {} is not a valid number", choice.trim());
                    choice = String::new();
                    continue;
                }
            };
            if max_depth == 0 || max_depth > 10 {
                println!("Invalid depth");
                continue;
            }
            break;
        }
        SmartBot::build_robot(max_depth)
    }
}

impl SmartBot {
    pub fn build_robot(max_depth: u8) -> SmartBot {
        SmartBot {
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
            return MaybeWin(0);
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
            //println!("Always win for every opponent move, {} current depth col {}",
            //    current_depth, the_move);
            return AlwaysWin;
        }
        if loss_count == valid_move_count {
            //println!("Always lose for every move, {} current depth col {}",
            //    current_depth, the_move);
            return AlwaysLose;
        }
        let future_val = value_sum / valid_move_count as isize;
        MaybeWin((future_val + win_count as isize- loss_count as isize) / 2)
    }
}