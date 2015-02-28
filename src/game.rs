//! Our Game object.

//Imports/uses
extern crate cli;
use self::cli::interface::Interface;
use std::ascii::AsciiExt;
use player::Player;
use player::PLAYERTYPE;

//Slot constants
pub const SLOT_EMPTY: u8 = 0;
pub const SLOT_X: u8 = 1;
pub const SLOT_O: u8 = 4;

//Game state constants
const NO_END: u8 = 0;
const TIE: u8 = 1;
const WIN_X: u8 = 2;
const WIN_O: u8 = 3;

/// The structure of members in our Game
pub struct Game {
	pub cli: Interface,
	pub running: bool,
	pub board: [u8; 9],
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
			board: [SLOT_EMPTY; 9],
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

				//Print the board
				self.cli.print(self.get_board_str() + "\n\n\n");

				let player_move = self.players[player].get_controller().make_move(&self.board);
				self.set_board_slot(player_move, player_type as u8);

				//Handle checking for the end of the game
				let end: u8 = check_end(&self.board);
				if end != NO_END {
					//Print the board
					self.cli.print(self.get_board_str());

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
			SLOT_X => "X",
			SLOT_O => "O",
			_ => " "
		}
	}

	/// Set a given board slot
	fn set_board_slot(&mut self, pos: usize, input: u8) {
		self.board[pos] = input;
	}
}

/// Checks the board for a win/loss/tie scenario then outputs an matching message
///
/// Returns: a bool of whether or not the game has ended
fn check_end(board: &[u8;9]) -> u8 {

	//If X has won
	if board[0] + board[1] + board[2] == SLOT_X * 3 //Top
	|| board[0] + board[3] + board[6] == SLOT_X * 3 //Left
	|| board[2] + board[5] + board[8] == SLOT_X * 3 //Right
	|| board[6] + board[7] + board[8] == SLOT_X * 3 //Bottom
	|| board[0] + board[4] + board[8] == SLOT_X * 3 //Diag left to right
	|| board[2] + board[4] + board[6] == SLOT_X * 3 { //Diag right to left
		println!("{}", (board[0] + board[1] + board[2]));
		return WIN_X;
	}

	//If O has won
	if board[0] + board[1] + board[2] == SLOT_O * 3 //Top
	|| board[0] + board[3] + board[6] == SLOT_O * 3 //Left
	|| board[2] + board[5] + board[8] == SLOT_O * 3 //Right
	|| board[6] + board[7] + board[8] == SLOT_O * 3 //Bottom
	|| board[0] + board[4] + board[8] == SLOT_O * 3 //Diag left to right
	|| board[2] + board[4] + board[6] == SLOT_O * 3 { //Diag right to left
		return WIN_O;
	}

	//If all of the pieces are used
	for slot in 0..9 {
		if board[slot] == SLOT_EMPTY {
			return NO_END; //No win/loss/tie yet
		}
	}

	TIE
}
