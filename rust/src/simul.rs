use crate::{
	framedata::CBox,
	input::FgInput,
	player::{Player, PlayerState},
	timer::Timer,
};
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(no_init)]
pub struct Match {
	timer: Timer,
	rounds: u8,
	player1: Player,
	player2: Player,
	state: GameState,
}

#[godot_api]
impl Match {
	pub const STAGE_LEN: i16 = 1530;
	const PLAYER_START: i16 = 400;
	#[allow(unused)]
	const ROUND_TIME: u16 = 3600;
	const ROUND_START_LEN: u8 = 90;
	const HITSTOP_LEN: u8 = 15;
	const ROUND_END_LEN: u8 = 60;

	#[func]
	pub fn gd_new(p1_bot: bool, p2_bot: bool) -> Gd<Self> {
		Gd::from_object(Self::new(p1_bot, p2_bot))
	}

	pub const fn new(p1_bot: bool, p2_bot: bool) -> Self {
		Match {
			timer: Timer::Limited(Self::ROUND_TIME),
			//timer: Timer::Unlimited,
			rounds: 0,
			player1: Player::new(Self::starting_position(true), p1_bot),
			player2: Player::new(Self::starting_position(false), p2_bot),
			state: GameState::RoundStart(Self::ROUND_START_LEN),
		}
	}

	#[func]
	pub fn new_round(&mut self) {
		self.player1.reset(Self::starting_position(true));
		self.player2.reset(Self::starting_position(false));

		*self = Match {
			player1: self.player1.clone(),
			player2: self.player2.clone(),
			// p1_bot and p2_bot does not matter
			..Match::new(false, false)
		};
	}

	#[func]
	pub fn frame_update(&mut self, input1: Gd<FgInput>, input2: Gd<FgInput>) -> Result {
		self.player1.counter_hit = false;
		self.player2.counter_hit = false;

		let input1 = *input1.bind();
		let input2 = *input2.bind();

		self.player1.set_input(input1);
		self.player2.set_input(input2);

		if !matches!(self.state, GameState::Hitstop(_)) {
			self.player1.update_buffer();
			self.player2.update_buffer();
		}

		self.state = self.state.step();

		if matches!(self.state, GameState::RoundFinish) {
			let res = self.end_result();
			self.update_wins(res);
			return res;
		}

		if !matches!(self.state, GameState::Active) {
			return Result::Pause;
		};

		self.timer = self.timer.step();

		self.combat_update();

		if self.player1.is_dead() || self.player2.is_dead() {
			self.state = GameState::RoundEnd(Self::ROUND_END_LEN);
			return Result::Continue;
		}

		if self.timer.is_over() {
			let res = self.end_result();
			self.update_wins(res);
			return res;
		}

		Result::Continue
	}

	fn combat_update(&mut self) {
		// Update char action
		self.player1.update_state();
		self.player2.update_state();

		// Get active movedata
		let p1_move = self.player1.update_move();
		let p2_move = self.player2.update_move();

		// Update movement
		self.position_update(p1_move.data.speed, p2_move.data.speed);

		// Update char collision
		self.collision_update(p1_move.data.collision, p2_move.data.collision);

		let p2_hit = if !self.player1.get_hit() {
			Self::hitbox_hurtbox_collision(
				&p1_move.data.hitbox,
				&p2_move.data.hurtbox,
				self.p1_pos(),
				self.p2_pos(),
				false,
			)
		} else {
			false
		};
		let p1_hit = if !self.player2.get_hit() {
			Self::hitbox_hurtbox_collision(
				&p2_move.data.hitbox,
				&p1_move.data.hurtbox,
				self.p2_pos(),
				self.p1_pos(),
				true,
			)
		} else {
			false
		};

		if p2_hit {
			self.player2
				.get_attacked(p1_move.data.ender, p1_move.data.low);
			self.player1.set_hit();
		}
		if p1_hit {
			self.player1
				.get_attacked(p2_move.data.ender, p2_move.data.low);
			self.player2.set_hit();
		}

		self.state = if p2_hit || p1_hit {
			GameState::Hitstop(Self::HITSTOP_LEN)
		} else {
			self.state
		}
	}

	#[inline]
	fn position_update(&mut self, p1_move: i16, p2_move: i16) {
		self.player1.move_position(p1_move);
		self.player2.move_position(-p2_move);
	}

	#[inline]
	fn collision_update(&mut self, p1_col: CBox, p2_col: CBox) {
		let offsetx1 = self.player1.position;
		let offsetx2 = self.player2.position;

		if p1_col.overlap(offsetx1, -p2_col, offsetx2) {
			let amount = p1_col.overlap_amount(offsetx1, -p2_col, offsetx2);

			self.player1.move_position(-amount);
			self.player2.move_position(amount);
		}
	}

	#[inline]
	const fn starting_position(player1: bool) -> i16 {
		if player1 {
			Self::PLAYER_START
		} else {
			Self::STAGE_LEN - Self::PLAYER_START
		}
	}

	#[inline]
	fn end_result(&self) -> Result {
		let player1 = &self.player1;
		let player2 = &self.player2;

		let p1down = player1.is_dead();
		let p2down = player2.is_dead();

		if p1down && p2down {
			return Result::Draw;
		} else if p1down {
			return Result::Player2;
		} else if p2down {
			return Result::Player1;
		}

		match player1.guard_points.cmp(&player2.guard_points) {
			std::cmp::Ordering::Greater => Result::Player1,
			std::cmp::Ordering::Less => Result::Player2,
			std::cmp::Ordering::Equal => {
				match self
					.player_relative_pos(true)
					.cmp(&self.player_relative_pos(false))
				{
					std::cmp::Ordering::Greater => Result::Player1,
					std::cmp::Ordering::Less => Result::Player2,
					std::cmp::Ordering::Equal => Result::Draw,
				}
			}
		}
	}

	#[inline]
	const fn update_wins(&mut self, result: Result) {
		match result {
			Result::Continue => (),
			Result::Pause => (),
			Result::Timeout => (),
			Result::Player1 => {
				self.player1.wins += 1;
			}
			Result::Player2 => {
				self.player2.wins += 1;
			}
			Result::Draw => {
				self.player1.wins += 1;
				self.player2.wins += 1;
			}
		}
	}

	#[inline]
	fn hitbox_hurtbox_collision(
		hitbox: &Option<CBox>,
		hurtbox: &[Option<CBox>],
		p1_pos: i16,
		p2_pos: i16,
		inverse: bool,
	) -> bool {
		let overlap_check = |hit: CBox, hurt: CBox| {
			if inverse {
				(-hit).overlap(p1_pos, hurt, p2_pos)
			} else {
				(hit).overlap(p1_pos, -hurt, p2_pos)
			}
		};

		hitbox.iter().any(|hit| {
			hurtbox.iter()
				.filter_map(|a| *a)
				.any(|hurt| overlap_check(*hit, hurt))
		})
	}

	#[func]
	#[inline]
	pub fn player_relative_pos(&self, p1: bool) -> i16 {
		match p1 {
			true => self.player1.position,
			false => Self::STAGE_LEN - self.player2.position,
		}
	}

	#[func]
	#[inline]
	pub fn player_distance(&self) -> i16 {
		(self.player1.position - self.player2.position).abs()
	}

	#[func]
	pub fn p1_pos(&self) -> i16 {
		self.player1.position
	}

	#[func]
	pub fn p2_pos(&self) -> i16 {
		self.player2.position
	}

	#[func]
	pub fn p1_sprite(&self) -> GString {
		GString::from(self.player1.get_move().animation_frame)
	}

	#[func]
	pub fn p2_sprite(&self) -> GString {
		GString::from(self.player2.get_move().animation_frame)
	}

	#[func]
	pub fn p1_wins(&self) -> u8 {
		self.player1.wins
	}

	#[func]
	pub fn p2_wins(&self) -> u8 {
		self.player2.wins
	}

	#[func]
	pub fn player_block(&self, p1: bool) -> bool {
		// Prevent spamming on hitstop, round end and round finish
		match self.state {
			GameState::Hitstop(..Self::HITSTOP_LEN)
			| GameState::RoundEnd(..Self::ROUND_END_LEN)
			| GameState::RoundFinish => {
				return false;
			}
			_ => (),
		}

		match p1 {
			true => self.player1.is_blocking(),
			false => self.player2.is_blocking(),
		}
	}

	#[func]
	pub fn player_block_ender(&self, p1: bool) -> bool {
		// Prevent spamming on hitstop, round end and round finish
		match self.state {
			GameState::Hitstop(..Self::HITSTOP_LEN)
			| GameState::RoundEnd(..Self::ROUND_END_LEN)
			| GameState::RoundFinish => {
				return false;
			}
			_ => (),
		}

		match p1 {
			true => self.player1.is_blocking_ender(),
			false => self.player2.is_blocking_ender(),
		}
	}

	#[func]
	pub fn player_guard(&self, p1: bool) -> u8 {
		match p1 {
			true => self.player1.guard_points,
			false => self.player2.guard_points,
		}
	}

	#[func]
	pub fn player_state(&self, p1: bool) -> i64 {
		match p1 {
			true => self.player1.state_int(),
			false => self.player2.state_int(),
		}
	}

	#[func]
	pub fn player_state_len(&self, p1: bool) -> i64 {
		match p1 {
			true => self.player1.state_len(),
			false => self.player2.state_len(),
		}
	}

	#[func]
	pub fn player_counter(&self, p1: bool) -> bool {
		match p1 {
			true => self.player1.counter_hit,
			false => self.player2.counter_hit,
		}
	}

	#[func]
	pub fn player_hit(&self, p1: bool) -> bool {
		// Prevent spamming on hitstop, round end and round finish
		match self.state {
			GameState::Hitstop(..Self::HITSTOP_LEN)
			| GameState::RoundEnd(..Self::ROUND_END_LEN)
			| GameState::RoundFinish => {
				return false;
			}
			_ => (),
		}

		match p1 {
			true => self.player1.is_hit(),
			false => self.player2.is_hit(),
		}
	}

	#[func]
	pub fn player_guard_break(&self, p1: bool) -> bool {
		// Prevent spamming on hitstop, round end and round finish
		match self.state {
			GameState::Hitstop(..Self::HITSTOP_LEN)
			| GameState::RoundEnd(..Self::ROUND_END_LEN)
			| GameState::RoundFinish => {
				return false;
			}
			_ => (),
		}

		match p1 {
			true => self.player1.newly_guard_break(),
			false => self.player2.newly_guard_break(),
		}
	}

	#[func]
	pub fn player_dead(&self, p1: bool) -> bool {
		// Prevent spamming on hitstop, round end and round finish
		match self.state {
			GameState::Hitstop(..Self::HITSTOP_LEN)
			| GameState::RoundEnd(..Self::ROUND_END_LEN)
			| GameState::RoundFinish => {
				return false;
			}
			_ => (),
		}

		match p1 {
			true => self.player1.newly_dead(),
			false => self.player2.newly_dead(),
		}
	}

	#[func]
	pub fn player_hold(&self, p1: bool) -> u8 {
		match p1 {
			true => self.player1.hold_time(),
			false => self.player2.hold_time(),
		}
	}

	#[func]
	pub fn timer_sec(&self) -> u16 {
		self.timer.seconds()
	}

	#[func]
	pub fn rounds(&self) -> u8 {
		self.rounds
	}

	#[func]
	pub fn audio(&self) -> Vec<GString> {
		let mut res = Vec::new();

		// Prevent audio spamming on hitstop, round end and round finish
		match self.state {
			GameState::Hitstop(..Self::HITSTOP_LEN)
			| GameState::RoundEnd(..Self::ROUND_END_LEN)
			| GameState::RoundFinish => {
				return res;
			}
			_ => (),
		}

		if let Some(audio) = self.player1.get_audio() {
			res.push(GString::from(audio));
		};
		if let Some(audio) = self.player2.get_audio() {
			res.push(GString::from(audio));
		};

		res
	}

	#[func]
	pub fn continues(&self) -> bool {
		self.player1.wins < 3 && self.player2.wins < 3
	}

	#[func]
	pub fn state(&self) -> i64 {
		self.state.into()
	}

	#[func]
	pub fn state_len(&self) -> i64 {
		self.state.state_len() as i64
	}

	#[func]
	pub fn stage_len() -> i16 {
		Self::STAGE_LEN
	}

	#[func]
	pub fn player_obs(&self, p1: bool) -> Vec<f32> {
		let (player, opponent) = match p1 {
			true => (&self.player1, &self.player2),
			false => (&self.player2, &self.player1),
		};

		let mut res = vec![
			self.player_relative_pos(p1) as f32 / Self::STAGE_LEN as f32,
			self.player_relative_pos(!p1) as f32 / Self::STAGE_LEN as f32,
			self.player_distance() as f32 / Self::STAGE_LEN as f32,
			player.guard_points as f32 / 3f32,
			opponent.guard_points as f32 / 3f32,
			player.wins as f32 / 3f32,
			opponent.wins as f32 / 3f32,
			player.recovery() as f32 / 60f32,
			opponent.recovery() as f32 / 60f32,
			player.can_block() as i32 as f32,
			opponent.can_block() as i32 as f32,
			player.hold_time() as f32 / 60f32,
			self.state_len() as f32 / 60f32,
		];

		// To lessen the reallocations
		res.reserve(((PlayerState::STATE_COUNT * 2) + GameState::STATE_COUNT) as usize);

		// player state as one-hot encoding
		res.extend((0..PlayerState::STATE_COUNT)
			.map(|x| (x == player.state_int()) as i32 as f32));

		// opponent state as one-hot encoding
		res.extend((0..PlayerState::STATE_COUNT)
			.map(|x| (x == opponent.state_int()) as i32 as f32));

		// game state as one-hot encoding
		res.extend((0..GameState::STATE_COUNT).map(|x| (x == self.state()) as i32 as f32));

		res
	}

	#[func]
	pub fn punish_obs(&self, p1: bool) -> Vec<f32> {
		let (player, opponent) = match p1 {
			true => (&self.player1, &self.player2),
			false => (&self.player2, &self.player1),
		};

		vec![
			Self::can_punish_nnormal(player, opponent, p1) as i32 as f32,
			Self::can_punish_mnormal(player, opponent, p1) as i32 as f32,
			Self::can_punish_nspecial(player, opponent, p1) as i32 as f32,
			Self::can_punish_mspecial(player, opponent, p1) as i32 as f32,
		]
	}

	#[inline]
	pub fn can_punish_nnormal(player: &Player, opponent: &Player, inverse: bool) -> bool {
		const HYPO_ATTACK: CBox = CBox {
			offsetx: 140,
			offsety: 0,
			x: 159,
			y: 46,
		};

		player.can_attack()
			&& opponent.recovery_punishable() > 18
			&& Self::hitbox_hurtbox_collision(
				&Some(HYPO_ATTACK),
				&opponent.get_move().data.hurtbox,
				player.position,
				opponent.position,
				inverse,
			)
	}

	#[inline]
	pub fn can_punish_mnormal(player: &Player, opponent: &Player, inverse: bool) -> bool {
		const HYPO_ATTACK: CBox = CBox {
			offsetx: 130,
			offsety: 0,
			x: 130,
			y: 138,
		};

		player.can_attack()
			&& opponent.recovery_punishable() > 17
			&& Self::hitbox_hurtbox_collision(
				&Some(HYPO_ATTACK),
				&opponent.get_move().data.hurtbox,
				player.position,
				opponent.position,
				inverse,
			)
	}

	#[inline]
	pub fn can_punish_nspecial(player: &Player, opponent: &Player, inverse: bool) -> bool {
		const HYPO_ATTACK: CBox = CBox {
			offsetx: 258,
			offsety: 119,
			x: 158,
			y: 55,
		};

		player.can_attack()
			&& opponent.recovery_punishable() > 25
			&& Self::hitbox_hurtbox_collision(
				&Some(HYPO_ATTACK),
				&opponent.get_move().data.hurtbox,
				player.position,
				opponent.position,
				inverse,
			)
	}

	#[inline]
	pub fn can_punish_mspecial(player: &Player, opponent: &Player, inverse: bool) -> bool {
		const HYPO_ATTACK: CBox = CBox {
			offsetx: 95,
			offsety: 0,
			x: 95,
			y: 158,
		};

		player.can_attack()
			&& opponent.recovery_punishable() > 16
			&& Self::hitbox_hurtbox_collision(
				&Some(HYPO_ATTACK),
				&opponent.get_move().data.hurtbox,
				player.position,
				opponent.position,
				inverse,
			)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
	RoundStart(u8),
	Active,
	Hitstop(u8),
	RoundEnd(u8),
	RoundFinish,
}

impl GameState {
	pub const STATE_COUNT: i64 = 5;

	#[inline]
	pub fn step(self) -> Self {
		match self {
			GameState::RoundStart(mut time) => {
				time -= 1;

				if time > 0 {
					GameState::RoundStart(time)
				} else {
					GameState::Active
				}
			}
			GameState::Active => GameState::Active,
			GameState::Hitstop(mut time) => {
				time -= 1;

				if time > 0 {
					GameState::Hitstop(time)
				} else {
					GameState::Active
				}
			}
			GameState::RoundEnd(mut time) => {
				time -= 1;

				if time > 0 {
					GameState::RoundEnd(time)
				} else {
					GameState::RoundFinish
				}
			}
			GameState::RoundFinish => GameState::RoundFinish,
		}
	}

	#[inline]
	pub fn state_len(self) -> u8 {
		match self {
			GameState::RoundStart(f) => f,
			GameState::Active => 0,
			GameState::Hitstop(f) => f,
			GameState::RoundEnd(f) => f,
			GameState::RoundFinish => 0,
		}
	}
}

impl Into<i64> for GameState {
	#[inline]
	fn into(self) -> i64 {
		match self {
			GameState::RoundStart(_) => 0,
			GameState::Active => 1,
			GameState::Hitstop(_) => 2,
			GameState::RoundEnd(_) => 3,
			GameState::RoundFinish => 4,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, GodotConvert, Var, Export)]
#[godot(via = i64)]
pub enum Result {
	Continue,
	Pause,
	Player1,
	Player2,
	Draw,
	Timeout,
}
