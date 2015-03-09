use controllers::Controller;
use board::*;

/// Our Memory structure. To hold previous game result data
pub struct Memory {
	pub wins: Box<Vec<[u8;9]>>,
	pub ties: Box<Vec<[u8;9]>>,
	pub losses: Box<Vec<[u8;9]>>
}

/// Our AIController structure. This will make moves on a given board
pub struct AIController {
	pub memory: Memory
}

impl AIController{}

/// Implementation of Controller for our AIController
///
/// This will allow the game to use a given player's
/// controller methods.
impl Controller for AIController {

	/// Make a move on the given board that matches the given
	/// player type. The AIController version of this method will
	/// use previous game data to decide a non-losing move
	///
	/// Returns: usize of the slot chosen to move into
	fn make_move(&mut self, board: &Board, win_type: u8) -> usize {

		//Select first winning move
		for slot in 0..9 {
			if board.get_board_slot(slot) == SLOT_EMPTY {
				if board.is_winning_move(slot, end_type) {
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

	/// Registering this controller will play games against itself
	/// and use the data there as the previous game data. This way
	/// when a game starts against a player, the AI isn't without
	/// any win/loss/tie data
	fn register(&mut self) {
	}
}