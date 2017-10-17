extern crate connect_four_engine;

use std::io;
use connect_four_engine::game;
use game::Game;
use game::players::Player;
use game::players::human::Human;
use game::players::random::Random;
use game::players::robot::Robot;

fn main() {
    match get_player(1) {
        1 => choose_player_2(Human::build_player()),
        2 => choose_player_2(Robot::build_player()),
        _ => choose_player_2(Random::build_player())
    };
}

fn choose_player_2<T: Player>(player_1: T){
    match get_player(2) {
        1 => create_game(player_1, Human::build_player()),
        2 => create_game(player_1, Robot::build_player()),
        _ => create_game(player_1, Random::build_player())
    };
}

fn create_game<T: Player, U: Player>(player_1: T, player_2: U){
    let mut player_1_wins: usize = 0;
    let mut player_2_wins: usize = 0;
    let mut my_game = Game::new_game(player_1,
        player_2, 8, 8, 4);
    let num_games = run_num_games();
    for i in 0..num_games {
        println!("Playing game {}, score is X {} - O {}",
            i, player_1_wins, player_2_wins);
        if my_game.play_game() == 1 {
            player_1_wins += 1;
        } else {
            player_2_wins += 1;
        }
    }
    println!("In {} games, player X won {} and player O won {}",
        num_games, player_1_wins, player_2_wins);
}

fn get_player(p_num: u8) -> u8 {
    let mut choice = String::new();
    let mut choice_i: u8;
    loop {
        println!("For player {}, which type? (1 human, 2 Robot, 3 Random)", p_num);
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");
        choice_i = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Sorry, {} is not a valid number", choice.trim());
                choice = String::new();
                continue;
            }
        };
        if choice_i > 0 && choice_i < 4 {
            return choice_i;
        }
        println!("Invalid choice");
        choice = String::new();
    }
}

fn run_num_games() -> usize {
    let mut choice = String::new();
    let mut choice_i: usize;
    loop {
        println!("Play how many games?");
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");
        choice_i = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Sorry, {} is not a valid number", choice.trim());
                choice = String::new();
                continue;
            }
        };
        if choice_i > 0 {
            return choice_i;
        }
        println!("Invalid choice");
        choice = String::new();
    }
}