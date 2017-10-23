//Learn from results, store in redis
use std::io;
use std::{thread, time};
//use std::sync::{Mutex, Arc};
use rand;
use rand::Rng;
use scoped_threadpool::Pool;
use redis;
use redis::Commands;
use redis::Connection;
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

pub struct LearnBot {
    max_depth: u8,
    val_per_connection: u16,
    my_game_piece: GamePiece,
    move_history: Vec<String>,
    redis: Connection
}

struct MoveValueFor {
    move_val: MoveValue,
    column: u8,
    from_redis: bool
}

fn get_redis_connection() -> Connection {
    let client = match redis::Client::open("redis://127.0.0.1/"){
        Ok(c) => c,
        Err(_) => panic!("Failed to open redis")
    };
    for i in 0..10 {
        match client.get_connection(){
            Ok(c) => return c,
            Err(e) => println!("Failed attempt {} to create redis connection: {}", i, e)
        }
        thread::sleep(time::Duration::from_secs(2));
    }
    panic!("Failed to get redis connection after 10 tries");
}

fn set_redis_key(con: &Connection, key: &str, value: isize) -> redis::RedisResult<()> {
    let _ : () = try!(con.set(key, value));
    Ok(())
}

fn redis_get_num(con: &Connection, key: &str) -> Option<isize> {
    match con.get(key) {
        Ok(i) => Some(i),
        Err(_) => None
    }
}

impl Player for LearnBot {
    fn which_move(&mut self, board: &Board) -> u8 {
        self.my_game_piece = board.get_current_move();
        self.val_per_connection = 1000 / (3 * (board.connect_number as u16 - 1) - 1);
        let mut mymove = u8::max_value();
        let mut max_value: isize = isize::min_value();
        let mut possible_moves = Vec::new();
        let mut move_vals: Vec<MoveValueFor> = Vec::new();
        for col in 0..board.num_columns {
            move_vals.push(MoveValueFor { move_val: InvalidMove, column: col, from_redis: false});
        }
        let mut thread_pool = Pool::new(board.num_columns as u32);
        let position_key = board.string_representation();
        //println!("Starting threads");
        thread_pool.scoped(|scope| {
            for mvf in &mut move_vals {
                let md = self.max_depth;
                let gp = &self.my_game_piece;
                let my_key = format!("{}-{}", position_key, mvf.column.to_string());
                if let Some(move_val) = redis_get_num(&self.redis, &my_key) {
                    mvf.move_val = match move_val {
                        999999 => AlwaysWin,
                        -999999 => AlwaysLose,
                        -987654 => InvalidMove,
                        _ => MaybeWin(move_val)
                    };
                    mvf.from_redis = true;
                } else {
                    scope.execute(move || {
                        let derived_val = LearnBot::value_for_move(mvf.column, &board, md, gp);
                        //println!("Got value for {} in thread. Storing...", mvf.column);
                        mvf.move_val = derived_val;
                    });
                };
            }
        });
        //println!("Thread are done, checking values");
        for mvf in move_vals.iter() {
            if ! mvf.from_redis {
                let my_key = format!("{}-{}", position_key, mvf.column.to_string());
                let val_to_store = match mvf.move_val {
                    AlwaysWin => 999999,
                    AlwaysLose => -999999,
                    InvalidMove => -987654,
                    MaybeWin(d_val) => d_val
                };
                let _ = set_redis_key(&self.redis, &my_key, val_to_store);
            }
        }
        for move_val_for in move_vals.iter() {
            match move_val_for.move_val {
                AlwaysWin => {
                    println!("Got an AlwaysWin for move {}", move_val_for.column);
                    let my_move = format!("{}-{}", position_key,
                        move_val_for.column.to_string());
                    self.move_history.push(my_move);
                    return move_val_for.column; 
                },
                MaybeWin(move_val) => {
                    //println!("Got MaybeWin with val {} for {}",
                        //move_val, move_val_for.column);
                    if move_val > max_value {
                        //println!("{} is now the max value", move_val_for.column);
                        possible_moves = Vec::new();
                        max_value = move_val;
                        mymove = move_val_for.column;
                    } else if move_val == max_value {
                        //println!("{} has the same move_val, adding to possibles",
                        //    move_val_for.column);
                        possible_moves.push(move_val_for.column);
                    }
                },
                _ => ()
                //AlwaysLose => println!("Got AlwaysLose for {}", move_val_for.column),
                //InvalidMove => println!("Got invalid move for {}", move_val_for.column)
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
        let my_move_str = format!("{}-{}", position_key,
            mymove.to_string());
        self.move_history.push(my_move_str);
        mymove
    }

    fn build_player() -> LearnBot {
        let mut max_depth: u8;
        let mut choice = String::new();
        loop {
            println!("For LearnBot, how deep to look?");
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
        LearnBot::build_robot(max_depth)
    }

    fn loss(&mut self) {
        let redis_client = get_redis_connection();
        let mut loss_factor = 1000;
        for k in self.move_history.iter().rev() {
            if let Some(move_val) = redis_get_num(&redis_client, &k) {
                match move_val {
                    999999 => (),
                    -999999 => (),
                    -987654 => (),
                    _ => {
                        let _ = set_redis_key(&redis_client, &k, move_val - loss_factor);
                        ()
                    }
                };
            };
            loss_factor = loss_factor * 8 / 10;
        }
        self.move_history = Vec::new();
    }

    fn win(&mut self) {
        let redis_client = get_redis_connection();
        let mut win_factor = 1000;
        for k in self.move_history.iter().rev() {
            if let Some(move_val) = redis_get_num(&redis_client, &k) {
                match move_val {
                    999999 => (),
                    -999999 => (),
                    -987654 => (),
                    _ => {
                        let _ = set_redis_key(&redis_client, &k, move_val + win_factor);
                        ()
                    }
                };
            };
            win_factor = win_factor * 8 / 10;
        }
        self.move_history = Vec::new();
    }
}

impl LearnBot {
    pub fn build_robot(max_depth: u8) -> LearnBot {
        LearnBot {
            max_depth,
            my_game_piece: GamePiece::Dash,
            val_per_connection: 0,
            move_history: Vec::new(),
            redis: get_redis_connection()
        }
    }

    fn value_for_move(the_move: u8, board: &Board, depth_left: u8, gp: &GamePiece) -> MoveValue {
        if !board.valid_move(the_move) {
            return InvalidMove;
        }
        let is_my_move = *gp == board.get_current_move();
        let mut board_cp = board.clone();
        board_cp.play_piece(the_move);
        if board_cp.have_winner() > 0 {
            if is_my_move {
                return AlwaysWin;
            } else {
                return AlwaysLose;
            }
        }
        if depth_left == 0 {
            return MaybeWin(0);
        }
        let mut valid_move_count: u8 = 0;
        let mut win_count: u8 = 0;
        let mut loss_count: u8 = 0;
        let mut value_sum: isize = 0;
        for col in 0..board_cp.num_rows {
            match LearnBot::value_for_move(col, &board_cp, depth_left - 1, gp) {
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