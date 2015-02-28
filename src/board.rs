//Slot types
pub const SLOT_EMPTY: u8 = 0;
pub const SLOT_X: u8 = 1;
pub const SLOT_O: u8 = 4;

//Board state constants
pub const NO_END: u8 = 0;
pub const TIE: u8 = 1;
pub const WIN_X: u8 = 2;
pub const WIN_O: u8 = 3;

pub struct Board {
	slots: [u8; 9]
}

impl Board {
	pub fn new() -> Board {
		Board {
			slots: [SLOT_EMPTY; 9]
		}
	}

	/// Check if the desired move is valid
	///
	/// Returns: bool of valid or not
	pub fn valid_move(&self, pos: &u8) -> bool {
		if *pos < 9 {
			if self.slots[*pos as usize] == 0 {
				return true;
			}
		}
		false
	}

	/// Method to return the board
	///
	/// Returns: The board as a string
	pub fn get_board_str(&self) -> String {
		let mut board: String = "".to_string();
		for _ in 0..100 {
			//board = board + "\n";
		}
		board = board +
			"     |     |     \n" +
			"  " + self.get_board_slot_type(0) + "  " + //top-left
			"|  " + self.get_board_slot_type(1) + "  |" + //top-center
			"  " + self.get_board_slot_type(2) + "  \n" + //top-right
			"_____|_____|_____\n" +
			"     |     |     \n" +
			"  " + self.get_board_slot_type(3) + "  " + //top-left
			"|  " + self.get_board_slot_type(4) + "  |" + //top-center
			"  " + self.get_board_slot_type(5) + "  \n" + //top-right
			"_____|_____|_____\n" +
			"     |     |     \n" +
			"  " + self.get_board_slot_type(6) + "  " + //top-left
			"|  " + self.get_board_slot_type(7) + "  |" + //top-center
			"  " + self.get_board_slot_type(8) + "  \n" + //top-right
			"     |     |     \n";
		board
	}

	/// Set a given board slot
	pub fn get_board_slot(&self, pos: usize) -> u8 {
		self.slots[pos]
	}

	/// Set a given board slot
	pub fn set_board_slot(&mut self, pos: usize, input: u8) {
		self.slots[pos] = input;
	}

	/// Get the visual representation of a board slot
	///
	/// Returns: A String of "X", "O", " "
	pub fn get_board_slot_type(&self, pos: usize) -> &'static str {
		match self.slots[pos] {
			SLOT_X => "X",
			SLOT_O => "O",
			_ => " "
		}
	}

	pub fn is_win(&self, board: [u8; 9]) -> u8 {

		//If X has won
		if board[0] + board[1] + board[2] == SLOT_X * 3 //Top
		|| board[0] + board[3] + board[6] == SLOT_X * 3 //Left
		|| board[2] + board[5] + board[8] == SLOT_X * 3 //Right
		|| board[6] + board[7] + board[8] == SLOT_X * 3 //Bottom
		|| board[1] + board[4] + board[7] == SLOT_X * 3 //Top-Bottom center
		|| board[3] + board[4] + board[5] == SLOT_X * 3 //Left-Right center
		|| board[0] + board[4] + board[8] == SLOT_X * 3 //Diag left to right
		|| board[2] + board[4] + board[6] == SLOT_X * 3 { //Diag right to left
			return WIN_X;
		}

		//If O has won
		if board[0] + board[1] + board[2] == SLOT_O * 3 //Top
		|| board[0] + board[3] + board[6] == SLOT_O * 3 //Left
		|| board[2] + board[5] + board[8] == SLOT_O * 3 //Right
		|| board[6] + board[7] + board[8] == SLOT_O * 3 //Bottom
		|| board[1] + board[4] + board[7] == SLOT_O * 3 //Top-Bottom center
		|| board[3] + board[4] + board[5] == SLOT_O * 3 //Left-Right center
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

	pub fn is_winning_move(&self, pos: usize, win_type: u8) -> bool {
		println!("Hit is_winning_move");
		let mut board: [u8; 9] = self.slots.clone();
		if board[pos] == SLOT_EMPTY {
			println!("Found empty slot {}", pos);
			board[pos] = match win_type {
				WIN_O => SLOT_O,
				_ => SLOT_X
			};
			if self.is_win(board) == win_type {
				println!("Found winning slot {}", pos);
				return true;
			}
			println!("Didn't find win. Got {} instead", self.is_win(board));
		}
		false
	}

	/// Checks the board for a win/loss/tie scenario then outputs an matching message
	///
	/// Returns: a bool of whether or not the game has ended
	pub fn check_end(&self) -> u8 {
		self.is_win(self.slots)
	}
}