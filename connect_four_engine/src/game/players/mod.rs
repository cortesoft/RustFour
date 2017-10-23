// players module

pub mod human;
pub mod random;
pub mod robot;
pub mod smartbot;
pub mod learnbot;

use super::board::*;

pub trait Player {
    fn which_move(&mut self, board: &Board) -> u8;
    fn build_player() -> Self;

    fn loss(&mut self) {
    }

    fn win(&mut self) {
    }
}