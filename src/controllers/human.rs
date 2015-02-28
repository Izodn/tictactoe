extern crate cli;
use self::cli::interface::Interface;
use controllers::Controller;

pub struct HumanController {
	pub cli: Interface
}

impl HumanController{}

impl Controller for HumanController {
	fn make_move(&self, board: &[u8; 9]) -> usize {
		let mut number: u8;
		loop {
			let mut user_move: String = "".to_string();

			self.cli.prompt("Your move?".to_string(), &mut user_move);

			let input: Option<u8> = user_move.trim().parse().ok();
			number = match input {
				Some(num) => num,
				None => 10 // 10 = invalid. There are 0 - 9 places on board
			};

			if self.valid_move(&number, &board) {
				break;
			} else {
				self.cli.print("I'm sorry, that is not a valid move.".to_string());
			}
		}
		number as usize
	}
}