pub mod human;
pub mod ai;
use self::human::HumanController;
use self::ai::AIController;

pub trait Controller {
	fn test(&self){
		println!("Hit Controller");
	}
	fn make_move(&self, _: &[u8; 9]) -> usize { 0 }

	/// Check if the desired move is valid
	///
	/// Returns: bool of valid or not
	fn valid_move(&self, pos: &u8, board: &[u8; 9]) -> bool {
		if *pos < 9 {
			if board[*pos as usize] == 0 {
				return true;
			}
		}
		false
	}
}

impl Controller for HumanController {
	fn test(&self) {
		println!("Hit HumanController");
	}
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
impl Controller for AIController {
	fn test(&self) {
		println!("Hit AIController");
	}
}
