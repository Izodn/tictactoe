//! Tic-Tac-Toe
pub mod game;
pub mod player;
pub mod controllers;
pub mod board;
use game::Game;

fn main() {
	let mut game: Game = Game::new();
	game.start();
}
