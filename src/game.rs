//! Our Game object.

//Imports/uses
extern crate cli;
use self::cli::interface::Interface;
use std::ascii::AsciiExt;

/// The structure of members in our Game
pub struct Game {
	cli: Interface,
	running: bool,
	board: [u8; 9]
}

/// Implementation of our Game
impl Game {

	/// Create a new Game object
	///
	/// Returns: a Game
	pub fn new() -> Game {
		Game {
			cli: Interface::new(),
			running: false,
			board: [0; 9]
		}
	}

	/// See if the player wants to start, then start or quit
	#[allow(unused_assignments)] //undecided isn't /read/ but it's observed as a loop condition
	pub fn start(&mut self) {
		let mut start_game: String = "".to_string();
		let mut undecided: bool = true;

		while undecided {
			undecided = false;

			//Ask the user to proceed
			self.cli.prompt("Would you like to play a game? (y/N)".to_string(), &mut start_game);

			if "y".to_string() == start_game.to_ascii_lowercase() {
				self.running = true;
				break;
			} else if "n".to_string() == start_game.to_ascii_lowercase() {
				self.running = false;
				break;
			} else {
				self.cli.print("I'm sorry, I didn't catch that.".to_string());
				undecided = true;
			}
		}

		//If the user chose to not play, return from this method, thus ending the program
		if !self.running {
			return;
		}

		self.main_loop();
	}

	/// The main game loop
	/// Here's where all of the move handling, ai, etc. logic will take place
	#[allow(unused_assignments)] //number isn't /read/ but its address is passed to a function
	fn main_loop(&mut self) {

		while self.running {

			//Print the board
			self.cli.print(self.get_board_str() + "\n\n\n");

			//User move
			while self.running {

				let mut user_move: String = "".to_string();
				let mut number: u8 = 10;

				self.cli.prompt("Your move?".to_string(), &mut user_move);

				let input: Option<u8> = user_move.trim().parse().ok();
				number = match input {
					Some(num) => num,
					None => 10 // 10 = invalid. There are 0 - 9 places on board
				};

				if self.valid_move(&number) {
					//Register user move
					self.set_board_slot(number as usize, 1);
					break;
				} else {
					self.cli.print("I'm sorry, that is not a valid move.".to_string());
				}
			}
		}
	}

	/// Check if the desired move is valid
	///
	/// Returns: bool of valid or not
	fn valid_move(&self, pos: &u8) -> bool {
		if *pos < 9 {
			if self.board[*pos as usize] == 0 {
				return true;
			}
		}
		false
	}

	/// Method to return the board
	///
	/// Returns: The board as a string
	fn get_board_str(&self) -> String {
		let mut board: String = "".to_string();
		for _ in 0..100 {
			board = board + "\n";
		}
		board = board +
			"     |     |     \n" +
			"  " + self.get_board_slot(0) + "  " + //top-left
			"|  " + self.get_board_slot(1) + "  |" + //top-center
			"  " + self.get_board_slot(2) + "  \n" + //top-right
			"_____|_____|_____\n" +
			"     |     |     \n" +
			"  " + self.get_board_slot(3) + "  " + //top-left
			"|  " + self.get_board_slot(4) + "  |" + //top-center
			"  " + self.get_board_slot(5) + "  \n" + //top-right
			"_____|_____|_____\n" +
			"     |     |     \n" +
			"  " + self.get_board_slot(6) + "  " + //top-left
			"|  " + self.get_board_slot(7) + "  |" + //top-center
			"  " + self.get_board_slot(8) + "  \n" + //top-right
			"     |     |     \n";
		board
	}

	/// Get the visual representation of a board slot
	///
	/// Returns: A String of "X", "O", " "
	fn get_board_slot(&self, pos: usize) -> &'static str {
		match self.board[pos] {
			1 => "X",
			2 => "O",
			_ => " "
		}
	}

	/// Set a given board slot
	fn set_board_slot(&mut self, pos: usize, input: u8) {
		self.board[pos] = input;
	}
}