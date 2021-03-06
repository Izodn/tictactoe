//! Our Game object.

//Imports/uses
use cli;
use std::ascii::AsciiExt;
use player::Player;
use player::PLAYERTYPE;
use board::Board;
use board::SLOT_X;
use board::SLOT_O;
use board::NO_END;
use board::TIE;
use board::WIN_X;
use board::WIN_O;

#[derive(Clone)]
pub struct Move {
	pub slot: u8,
	pub slot_type: u8
}

#[derive(Clone)]
pub struct GameData {
	pub moves: Vec<Move>,
	pub game_status: u8
}

/// The structure of members in our Game
pub struct Game {
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
			start_game = cli::prompt("Would you like to play a game? (y/N)".to_string());

			if "y".to_string() == start_game.to_ascii_lowercase() {
				self.running = true;
				break;
			} else if "n".to_string() == start_game.to_ascii_lowercase() {
				self.running = false;
				break;
			} else {
				cli::print("I'm sorry, I didn't catch that.".to_string());
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

		//Register players
		for player in 0..2 {
			self.players[player].get_controller().register();
		}

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
				cli::print(self.board.get_board_str() + "\n\n\n");

				let player_move = self.players[player].get_controller().make_move(&self.board, win_type as u8);
				self.board.set_board_slot(player_move, player_type);

				//Handle checking for the end of the game
				let end: u8 = self.board.check_end();
				if end != NO_END {
					//Print the board
					cli::print(self.board.get_board_str());

					//Output the proper win/tie message
					match end {
						TIE => {
							cli::print("The game is a tie.".to_string());
						},
						WIN_X => {
							cli::print("X is the winner".to_string());
						},
						WIN_O => {
							cli::print("O is the winner.".to_string());
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
