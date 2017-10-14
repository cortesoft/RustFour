// players module

pub mod human;
pub mod random;

use super::board::*;

pub trait Player {
    fn which_move(&self, board: &Board) -> u8;
}