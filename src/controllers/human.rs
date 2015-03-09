extern crate cli;
use self::cli::interface::Interface;
use controllers::Controller;
use board::*;

/// Our HumanController structure. This will make moves on a given board
pub struct HumanController {
	pub cli: Interface
}

impl HumanController{}

/// Implementation of Controller for our HumanController
///
/// This will allow the game to use a given player's
/// controller methods.
impl Controller for HumanController {

	/// Make a move on the given board that matches the given
	/// player type. The HumanController version of this method
	/// will use user-input to make a move
	///
	/// Returns: usize of the slot chosen to move into
	fn make_move(&mut self, board: &Board, _: u8) -> usize {
		let mut number: u8;
		loop {
			let mut user_move: String = "".to_string();

			self.cli.prompt("Your move?".to_string(), &mut user_move);

			let input: Option<u8> = user_move.trim().parse().ok();
			number = match input {
				Some(num) => num,
				None => 10 // 10 = invalid. There are 0 - 9 places on board
			};

			if board.valid_move(&number) {
				break;
			} else {
				self.cli.print("I'm sorry, that is not a valid move.".to_string());
			}
		}
		number as usize
	}
}