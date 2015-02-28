//! Our Game object.

//Imports/uses
extern crate cli;
use self::cli::interface::Interface;
use std::ascii::AsciiExt;
use player::Player;
use player::PLAYERTYPE;
use board::*;

/// The structure of members in our Game
pub struct Game {
	pub cli: Interface,
	pub running: bool,
	pub board: Board,
	pub players: Vec<Player>
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
			board: Board::new(),
			players: Vec::<Player>::new()
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

		//Set the players
		self.players.push( Player::new(PLAYERTYPE::HUMAN) );
		self.players.push( Player::new(PLAYERTYPE::AI) );

		self.main_loop();
	}

	/// The main game loop
	/// Here's where all of the move handling, ai, etc. logic will take place
	#[allow(unused_assignments)] //number isn't /read/ but its address is passed to a function
	fn main_loop(&mut self) {

		while self.running {

			//Make moves
			for player in 0..2 {
				let player_type = match player {
					1 => SLOT_O,
					_ => SLOT_X
				};
				let win_type = match player {
					1 => WIN_O,
					_ => WIN_X
				};

				//Print the board
				self.cli.print(self.board.get_board_str() + "\n\n\n");

				let player_move = self.players[player].get_controller().make_move(&self.board, win_type as u8);
				self.board.set_board_slot(player_move, player_type as u8);

				//Handle checking for the end of the game
				let end: u8 = self.board.check_end();
				if end != NO_END {
					//Print the board
					self.cli.print(self.board.get_board_str());

					//Output the proper win/tie message
					match end {
						TIE => {
							self.cli.print("The game is a tie.".to_string());
						},
						WIN_X => {
							self.cli.print("X is the winner".to_string());
						},
						WIN_O => {
							self.cli.print("O is the winner.".to_string());
						},
						_ => {}
					}

					//Break out of the main loop
					self.running = false;
					break;
				}
			}
		}
	}
}
