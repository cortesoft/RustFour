//Just return random moves
use rand;
use rand::Rng;
use super::Player;
use super::*;

pub struct Random {}

impl Player for Random {
    fn which_move(&self, board: &Board) -> u8 {
        let mut mymove: u8 = rand::thread_rng().gen_range(0, 8);
        while !board.valid_move(mymove){
            mymove = rand::thread_rng().gen_range(0, 8);
        }
        mymove
    }
}