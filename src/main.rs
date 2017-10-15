extern crate connect_four_engine;
use connect_four_engine::game;
//use game::players::human::Human;
//use game::players::random::Random;
use game::players::robot::Robot;

fn main() {
    //let player1 = Human {
    //    name: String::from("Daniel")
    //};
    let player1 = Robot::build_robot(3);
    let player2 = Robot::build_robot(8);
    game::play(player1, player2);
}
