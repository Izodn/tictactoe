pub mod human;
pub mod ai;
use board::Board;

pub trait Controller {
	fn make_move(&self, _: &Board, _: u8) -> usize { 0 }
}
