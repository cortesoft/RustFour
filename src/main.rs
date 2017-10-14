extern crate connect_four_engine;
use connect_four_engine::game;
use game::players::human::Human;
use game::players::random::Random;

fn main() {
    let player1 = Human {
        name: String::from("Daniel")
    };
    let player2 = Random {};
    game::play(player1, player2);
}
