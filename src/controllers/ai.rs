extern crate cli;
use self::cli::interface::Interface;
use controllers::Controller;
use board::*;

pub struct AIController {
	pub cli: Interface
}

impl AIController{}

impl Controller for AIController {
	fn make_move(&self, board: &Board, player_type: u8) -> usize {

		//Select winning moves
		for slot in 0..9 {
			if board.get_board_slot(slot) == SLOT_EMPTY {
				if board.is_winning_move(slot, player_type) {
					return slot;
				}
			}
		}

		//Select first left-over move
		for slot in 0..9 {
			if board.get_board_slot(slot) == SLOT_EMPTY {
				return slot;
			}
		}

		//Couldn't decide? Shouldn't hit this
		0
	}
}