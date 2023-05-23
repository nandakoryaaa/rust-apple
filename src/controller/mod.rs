use crate::game::{ GameStateEvent, PlayerAnimationState };
use crate::input::{ Input, InputEvent };
use crate::model::{ Model, ModelDataMain };
use crate::xrand::XRand;

pub enum ColliderEvent {
	None,
	HitPlayer,
	HitGround,
	HitBasket
}

pub struct PlayerAppleCollider {
}

impl PlayerAppleCollider {
	pub fn test(& self, player_dir: i32, data: & ModelDataMain) -> ColliderEvent {
		if data.apple_y == 26 {
			if player_dir == -1 {
				if data.apple_x == data.player_x + 2 {
					return ColliderEvent::HitPlayer;
				}
			} else {
				if data.apple_x == data.player_x + 1 {
					return ColliderEvent::HitPlayer;
				}
			}
		} else if data.apple_y == 28 {
			if player_dir == -1 {
				if data.apple_x == data.player_x {
					return ColliderEvent::HitBasket;
				}
			} else {
				if data.apple_x == data.player_x + 3 {
					return ColliderEvent::HitBasket;
				}
			}
		} else if data.apple_y >= data.grid_h as i32 {
			return ColliderEvent::HitGround;
		}

		ColliderEvent::None
	}
}

pub trait Controller {
	fn update_input(& mut self, model: & mut Model, input: & dyn Input);
	fn update(& mut self, model: & mut Model) -> GameStateEvent;
	fn reset(& mut self);
}

pub struct ControllerTitle {
	pub evt: GameStateEvent
}

pub struct ControllerMenu {
	pub evt: GameStateEvent
}

pub struct ControllerMain {
	pub tick: i32,
	pub evt: GameStateEvent,
	pub player_dir: i32,
	pub player_step: i32,
	pub need_new_apple: bool,
	pub collider: PlayerAppleCollider,
	pub rand: XRand
}

pub struct ControllerGameOver {
	pub evt: GameStateEvent,
	pub phase: i32,
	pub tick: i32
}

impl Controller for ControllerTitle {
	fn update_input(& mut self, _model: & mut Model, input: & dyn Input) {
		self.evt = GameStateEvent::Empty;
		let evt: InputEvent = input.get_event();
		match evt {
			InputEvent::Continue => {
				self.evt = GameStateEvent::RunMenu;
			},
			_ => ()
		}
	}

	fn update(& mut self, _model: & mut Model) -> GameStateEvent {
		self.evt
	}

	fn reset(& mut self) {
		self.evt = GameStateEvent::Empty;
	}
}

impl Controller for ControllerMenu {
	fn update_input(& mut self, model: & mut Model, input: & dyn Input) {
		self.evt = GameStateEvent::Empty;
		let evt: InputEvent = input.get_event();
		match model {
			Model::ModelMenu { level, .. } => {
				match evt {
					InputEvent::ItemPrev => {
						if *level > 0 {
							*level -= 1;
						}
						self.evt = GameStateEvent::Run;
					}
					InputEvent::ItemNext => {
						if *level < 8 {
							*level += 1;
						}
						self.evt = GameStateEvent::Run;
					}
					InputEvent::ItemSelect => {
						self.evt = GameStateEvent::RunMain;
					},
					_ => ()
				}
			},
			_ => ()
		}
	}

	fn update(& mut self, _model: & mut Model) -> GameStateEvent {
		let evt = self.evt;
		self.evt = GameStateEvent::Empty;

		evt
	}

	fn reset(& mut self) {
		self.evt = GameStateEvent::Empty;
	}
}

impl Controller for ControllerMain {
	fn update_input(& mut self, model: & mut Model, input: & dyn Input) {
		self.evt = GameStateEvent::Empty;
		let evt: InputEvent = input.get_event();
		match model {
			Model::ModelMain { data } => {
				match evt {
					InputEvent::MoveLeft => {
						if self.player_step != -1 {
							data.player_state = PlayerAnimationState::MoveLeft;
							if self.player_step == 1 {
								data.player_x -= 2;
							}
							self.player_dir = -1;
							self.player_step = -1;
							self.evt = GameStateEvent::Run;
						}
					},
					InputEvent::MoveRight => {
						if self.player_step != 1 {
							data.player_state = PlayerAnimationState::MoveRight;
							if self.player_step == -1 {
								data.player_x += 2;
							}
							self.player_dir = 1;
							self.player_step = 1;
							self.evt = GameStateEvent::Run;
						}
					},
					InputEvent::Stop => {
						if self.player_step == -1 {
							data.player_state = PlayerAnimationState::StandLeft;
							self.evt = GameStateEvent::Run;
						} else if self.player_step == 1 {
							data.player_state = PlayerAnimationState::StandRight;
							self.evt = GameStateEvent::Run;
						}
						self.player_step = 0;
					},
					_ => ()
				}
			},
			_ => ()
		}
	}

	fn update(& mut self, model: & mut Model) -> GameStateEvent {
		if self.tick > 0 {
			self.tick -= 1;
			let evt = self.evt;
			self.evt = GameStateEvent::Empty;
			return evt;
		}

		match model {
			Model::ModelMain { data	} => {
				self.tick = 8 - data.level;

				if self.player_step != 0 {
					data.player_x += self.player_step;
					if data.player_x < 0 {
						data.player_x = 0;
					} else if data.player_x > (data.grid_w - 4) as i32 {
						data.player_x = (data.grid_w - 4) as i32;
					}
				}

				if self.need_new_apple {
					data.apple_y = 0;
					data.apple_x = self.rand.randint(2, data.grid_w - 4) as i32;
					self.need_new_apple = false;
				}

				data.apple_y += 1;

				let collider_event = self.collider.test(self.player_dir, data) as i32;
				if collider_event == ColliderEvent::HitPlayer as i32 {
					return GameStateEvent::RunGameOver;
				} else if collider_event == ColliderEvent::HitGround as i32 {
					self.need_new_apple = true;
					data.apples_lost += 1;
					if data.apples_lost == data.apple_cnt {
						return GameStateEvent::RunGameOver;
					}
				} else if collider_event == ColliderEvent::HitBasket as i32 {
					self.need_new_apple = true;
					data.apples_collected += 1;
					if data.apples_collected == data.apple_cnt {
						return GameStateEvent::RunGameOver;
					}
				}

				return GameStateEvent::Run;
			},
			_ => {
				return GameStateEvent::Empty;
			}
		}
	}

	fn reset(& mut self) {
		self.evt = GameStateEvent::Empty;
		self.tick = 0;
		self.player_dir = 0;
		self.player_step = 0;
		self.need_new_apple = true;
	}
}

impl Controller for ControllerGameOver {
	fn update_input(& mut self, _model: & mut Model, input: & dyn Input) {
		self.evt = GameStateEvent::Empty;
		let evt: InputEvent = input.get_event();
		match evt {
			InputEvent::Continue => {
				self.evt = GameStateEvent::RunTitle;
			},
			_ => ()
		}
	}

	fn update(& mut self, _model: & mut Model) -> GameStateEvent {
		if self.phase > 0 {
			self.phase -= 1;
			if self.tick > 0 {
				self.tick -= 1;
				return GameStateEvent::Empty;
			}
			self.tick = 5;
			return GameStateEvent::Run
		}

		self.evt
	}

	fn reset(& mut self) {
		self.evt = GameStateEvent::Empty;
		self.phase = 60;
	}
}
