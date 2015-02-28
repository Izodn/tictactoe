pub mod human;
pub mod ai;

pub trait Controller {
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
