//Just return smart moves
use std::io;
use rand;
use rand::Rng;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
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

pub struct Robot<'a> {
    max_depth: u8,
    val_per_connection: u16,
    my_game_piece: GamePiece,
    threads: Vec<JoinHandle<()>>,
    thread_pipes: Vec<Sender<(&'a Robot, u8, &'a Board)>>,
    receiver: Receiver<(MoveValue, u8)>,
    channel_sender: Sender<(MoveValue, u8)>
}

impl Player for Robot {
    fn which_move(&mut self, board: &Board) -> u8 {
        self.my_game_piece = board.get_current_move();
        self.val_per_connection = 1000 / (3 * (board.connect_number as u16 - 1) - 1);
        let mut mymove = u8::max_value();
        let mut max_value: isize = isize::min_value();
        let mut possible_moves = Vec::new();
        self.create_threads(board.num_rows);
        let mut move_vals: Vec<(MoveValue, u8)> = Vec::new();
        for thread_col in 0..board.num_rows {
            let to_send = (self, thread_col, board);
            self.thread_pipes[thread_col as usize].send(to_send).unwrap();
        }
        //Now listen for responses
        while move_vals.len() < board.num_rows as usize {
            let resp: (MoveValue, u8) = self.receiver.recv().unwrap();
            println!("Got move back for {}", resp.1);
            move_vals.push(resp);
        }
        for move_val in move_vals.iter() {
            let (val, col) = move_val;
            match val {
                AlwaysWin => {
                    println!("Got an AlwaysWin for move {}", col);
                    return col; 
                },
                MaybeWin(move_val) => {
                    println!("Got MaybeWin with val {} for {}", move_val, col);
                    if move_val > max_value {
                        println!("{} is now the max value", col);
                        possible_moves = Vec::new();
                        max_value = move_val;
                        mymove = col;
                    } else if move_val == max_value {
                        println!("{} has the same move_val, adding to possibles", col);
                        possible_moves.push(col);
                    }
                },
                AlwaysLose => println!("Got AlwaysLose for {}", col),
                InvalidMove => println!("Got invalid move for {}", col)
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

    fn build_player() -> Robot {
        let mut max_depth: u8;
        let mut choice = String::new();
        loop {
            println!("For robot, how deep to look?");
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
        Robot::build_robot(max_depth)
    }
}

impl Robot {
    pub fn build_robot(max_depth: u8) -> Robot {
        let (tx, rx) = mpsc::channel();
        Robot {
            max_depth,
            my_game_piece: GamePiece::Dash,
            val_per_connection: 0,
            threads: Vec::new(),
            thread_pipes: Vec::new(),
            receiver: rx,
            channel_sender: tx
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

    fn create_threads(&mut self, num_threads: u8) {
        while self.threads.len() < num_threads as usize {
            //Communication with this thread
            let (tx, rx) = mpsc::channel();
            //The sender for this thread back to main
            let my_sender = self.channel_sender.clone();
            let my_handle = thread::spawn(move||{
                loop {
                    let (rob, the_move, board) =
                        rx.recv().unwrap();
                    let resp = (rob.value_for_move(the_move, board, 0), the_move);
                    my_sender.send(resp).unwrap();
                }
            });
            self.threads.push(my_handle);
            self.thread_pipes.push(tx);
        }
    }
}