extern crate cli;
use controllers::Controller;
use controllers::human::HumanController;
use controllers::ai::AIController;
use self::cli::interface::Interface;

pub enum PLAYERTYPE {
	HUMAN,
	AI
}
pub struct Player {
	controller: Box<Controller>
}
impl Player {
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
						cli: Interface::new()
					})
				}
			},
		}
	}
	pub fn get_controller(&self) -> &Box<Controller> {
		&self.controller
	}
}
