pub mod human;
pub mod ai;
use board::Board;

/// Our Controller trait. This trait will be used to
/// decide a player's move.
pub trait Controller {

	/// Make a move on a given board
	fn make_move(&mut self, _: &Board, _: u8) -> usize { 0 }

	/// Run any necessary pre-game things
	fn register(&mut self) {}
}
