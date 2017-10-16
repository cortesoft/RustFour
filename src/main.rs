extern crate connect_four_engine;

use std::io;
use connect_four_engine::game;
use game::players::Player;
use game::players::human::Human;
use game::players::random::Random;
use game::players::robot::Robot;

fn main() {
    let players = (get_player(1), get_player(2));
    match players {
        (1,1) => game::play(Human::build_player(),Human::build_player()),
        (1,2) => game::play(Human::build_player(),Robot::build_player()),
        (1,3) => game::play(Human::build_player(),Random::build_player()),
        (2,1) => game::play(Robot::build_player(),Human::build_player()),
        (2,2) => game::play(Robot::build_player(),Robot::build_player()),
        (2,3) => game::play(Robot::build_player(),Random::build_player()),
        (3,1) => game::play(Random::build_player(),Human::build_player()),
        (3,2) => game::play(Random::build_player(),Robot::build_player()),
        (3,3) => game::play(Random::build_player(),Random::build_player()),
        (_,_) => panic!("Bad combo!")
    }
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