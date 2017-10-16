// players module

pub mod human;
pub mod random;
pub mod robot;

use super::board::*;

pub trait Player {
    fn which_move(&mut self, board: &Board) -> u8;
    fn build_player() -> Self;
}