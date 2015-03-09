use controllers::Controller;
use board::*;

/// Our Memory structure. To hold previous game result data
pub struct Memory {
	pub games: Box<Vec<GameData>>
}

pub struct GameData {
	pub moves: Vec<Move>,
	pub game_status: u8
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
	fn make_move(&mut self, board: &Board, end_type: u8) -> usize {

		//Select first winning move
		for slot in 0..9 {
			if board.get_board_slot(slot) == SLOT_EMPTY {
				if board.is_winning_move(slot, end_type) {
					return slot;
				}
			}
		}

		//Get most popular non-losing move from memory
		let popular_move: usize = decide_move(&self.memory, board, end_type);
		if board.valid_move(&(popular_move as u8)) {
			return popular_move
		}

		//Choose the next open slot
		for slot in 0..9 {
			if board.valid_move(&(slot as u8)) {
				return slot;
			}
		}
		0
	}

	/// Registering this controller will play games against itself
	/// and use the data there as the previous game data. This way
	/// when a game starts against a player, the AI isn't without
	/// any win/loss/tie data
	fn register(&mut self) {
		for _ in 0..1 {
			let mut board: Board = Board::new();
			let mut game: GameData = GameData {
				moves: Vec::<Move>::new(),
				game_status: NO_END
			};

			let mut slot_type: u8 = SLOT_X;

			loop {
				let win_type = match slot_type {
					SLOT_O => WIN_O,
					_ => WIN_X
				};

				let player_move: usize = self.make_move(&board, win_type);
				board.set_board_slot(player_move, slot_type);
				game.moves.push(
					Move {
						slot: player_move as u8,
						slot_type: slot_type
					}
				);

				let end_type: u8 = board.check_end();
				if end_type != NO_END {
					game.game_status = end_type;
					break;
				}

				if slot_type == SLOT_X {
					slot_type = SLOT_O;
				} else {
					slot_type = SLOT_X;
				}
			}
			self.memory.games.push(game);
		}
	}
}

fn decide_move(memory: &Memory, board: &Board, end_type: u8) -> usize {
	let mut popularity: [u64;9] = [0;9];
	let relevant_games: Vec<GameData> = get_relevant_games(memory, board);

	//Calculate popularity of winning moves
	for game in relevant_games {
		if game.game_status == end_type {
			popularity[
				game.moves[
					get_turn_number(board)
				].slot as usize
			] += 3;
		} else if game.game_status == TIE {
			popularity[
				game.moves[
					get_turn_number(board)
				].slot as usize
			] += 1;
		} else {
			popularity[
				game.moves[
					get_turn_number(board)
				].slot as usize
			] -= 1;
		}
	}
	let mut popular_move = 0;
	for cur_move in 0..9 {
		if popularity[cur_move] > popularity[popular_move] {
			popular_move = cur_move
		}
	}
	popular_move
}
fn get_relevant_games(memory: &Memory, board: &Board) -> Vec<GameData> {
	let mut relevant_games = Vec::<GameData>::new();

	//Save the turn. If this is the very first turn, this will be 1
	let turn: usize = get_turn_number(&board)-1;

	//Iterate over every known game
	for game in 0..(*memory.games).len() {
		if memory.games[game].game_status == NO_END {
			break;
		}

		//Is this game relevant
		let mut relevant_game: bool = true;

		//For every turn so far
		for turn_iterator in 0..turn {

			//If the following move isn't in the current set
			if memory.games[game].moves[turn_iterator].slot != board.moves[turn_iterator].slot {
				relevant_game = false;
				break;
			}
		}

		//If the game is still relevant
		if relevant_game {

			//Add the game to the pool
			relevant_games.push(
				GameData {
					moves: memory.games[game].moves.clone(),
					game_status: memory.games[game].game_status
				}
			);
		}
	}
	relevant_games
}
fn get_turn_number(board: &Board) -> usize {
	let mut turn_number: u8 = 0;
	for slot in 0..9 {
		if board.get_board_slot(slot) != SLOT_EMPTY {
			turn_number += 1;
		}
	}
	(turn_number + 1) as usize
}
