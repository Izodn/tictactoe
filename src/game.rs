//! Our Game object.

//Imports/uses
extern crate cli;
use self::cli::interface::Interface;
use std::ascii::AsciiExt;

/// The structure of members in our Game
pub struct Game {
	cli: Interface,
	running: bool
}

/// Implementation of our Game
impl Game {

	/// Create a new Game object
	///
	/// Returns: a Game
	pub fn new() -> Game {
		Game {
			cli: Interface::new(),
			running: false
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
	fn main_loop(&mut self) {

		while self.running {
			let mut user_move: String = "".to_string();
			self.cli.prompt("Your move".to_string(), &mut user_move);
		}
	}
}