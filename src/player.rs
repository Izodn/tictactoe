extern crate cli;
use controllers::Controller;
use controllers::human::HumanController;
use controllers::ai::AIController;
use controllers::ai::Memory;
use controllers::ai::GameData;
use self::cli::interface::Interface;

/// Enum of player types
pub enum PLAYERTYPE {
	HUMAN,
	AI
}

/// Our Player structure
pub struct Player {
	controller: Box<Controller>
}

/// Implementation of Player
impl Player {

	/// Creates a new player of a given type
	///
	/// Returns: Player A player with an AI or Human controller
	pub fn new(player_type: PLAYERTYPE) -> Player {
		match player_type {
			PLAYERTYPE::HUMAN => {
				Player {
					controller: Box::new(HumanController {
						cli: Interface::new()
					})
				}
			},
			PLAYERTYPE::AI => {
				Player {
					controller: Box::new(AIController {
						memory: Memory {
							games: Box::new(Vec::<GameData>::new())
						}
					})
				}
			},
		}
	}

	/// Get the player's controller
	///
	/// Returns: A mutable reference to a controller
	pub fn get_controller(&mut self) -> &mut Box<Controller + 'static> {
		&mut self.controller
	}
}
