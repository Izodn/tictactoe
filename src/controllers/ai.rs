extern crate cli;
use self::cli::interface::Interface;
use controllers::Controller;

pub struct AIController {
	pub cli: Interface
}

impl AIController{}

impl Controller for AIController {
	fn make_move(&self, board: &[u8; 9]) -> usize {
		for slot in 0..9 {
			if board[slot] == 0 {
				return slot;
			}
		}
		0
	}
}