use std::ops::{self};

#[derive(Clone, Copy)]
pub struct CBox {
	pub offsetx: i16,
	pub offsety: i16,
	pub x: i16,
	pub y: i16,
}

impl CBox {
	#[inline]
	pub const fn collision() -> Self {
		CBox {
			offsetx: 0,
			offsety: 0,
			x: 125,
			y: 153,
		}
	}

	#[inline]
	pub const fn base_hurtbox() -> Self {
		CBox {
			offsetx: 0,
			offsety: 0,
			x: 158,
			y: 184,
		}
	}

	#[inline]
	pub const fn guard_break_hurtbox() -> Self {
		CBox {
			offsetx: 0,
			offsety: 0,
			x: 184,
			y: 184,
		}
	}

	pub fn overlap(self, offsetx1: i16, other: CBox, offsetx2: i16) -> bool {
		let c1x1 = self.offsetx + offsetx1;
		let c1x2 = self.x + self.offsetx + offsetx1;
		let c2x1 = other.offsetx + offsetx2;
		let c2x2 = other.x + other.offsetx + offsetx2;
		let c1xrange = (c1x1.min(c1x2), c1x1.max(c1x2));
		let c2xrange = (c2x1.min(c2x2), c2x1.max(c2x2));

		if c1xrange.0 >= c2xrange.1 || c2xrange.0 >= c1xrange.1 {
			return false;
		}

		let c1y1 = self.offsety;
		let c1y2 = self.y + self.offsety;
		let c2y1 = other.offsety;
		let c2y2 = other.y + other.offsety;
		let c1yrange = (c1y1.min(c1y2), c1x1.max(c1y2));
		let c2yrange = (c2y1.min(c2y2), c2x1.max(c2y2));

		if c1yrange.0 >= c2yrange.1 || c2yrange.0 >= c1yrange.1 {
			return false;
		}

		true
	}

	#[inline]
	pub fn overlap_amount(&self, offsetx1: i16, other: CBox, offsetx2: i16) -> i16 {
		((self.x + self.offsetx + offsetx1) - (other.x + other.offsetx + offsetx2)) / 2
	}
}

impl ops::Mul<i16> for CBox {
	type Output = Self;

	fn mul(self, rhs: i16) -> Self::Output {
		CBox {
			x: self.x * rhs,
			offsetx: self.offsetx * rhs,
			..self
		}
	}
}

impl ops::Neg for CBox {
	type Output = Self;

	fn neg(self) -> Self::Output {
		CBox {
			x: -self.x,
			offsetx: -self.offsetx,
			..self
		}
	}
}

#[derive(Clone)]
pub struct FrameData {
	pub speed: i16,
	pub collision: CBox,
	// Both hitbox and hurtbox used to be stored in a Box<[CBox]>,
	// but to squeeze out every bit of CPU and memory dur,ng training, I changed it to this mess.
	// If arrayvec or tinyvec crates had const ways to do this, I would used them.
	pub hitbox: Option<CBox>,
	pub hurtbox: [Option<CBox>; 2],
	pub cancel: bool,
	pub ender: bool,
	pub low: bool,
}

impl FrameData {
	const fn default() -> Self {
		Self {
			speed: 0,
			collision: CBox::collision(),
			hitbox: None,
			hurtbox: [Some(CBox::base_hurtbox()), None],
			cancel: false,
			ender: false,
			low: false,
		}
	}
}

#[derive(Clone)]
pub struct MoveData {
	pub data: FrameData,
	pub animation_frame: &'static str,
	pub duration: u8,
}

pub const IDLE_DATA: [MoveData; 5] = [
	MoveData {
		data: FrameData::default(),
		animation_frame: "idle_0",
		duration: 6,
	},
	MoveData {
		data: FrameData::default(),
		animation_frame: "idle_1",
		duration: 3,
	},
	MoveData {
		data: FrameData::default(),
		animation_frame: "idle_2",
		duration: 6,
	},
	MoveData {
		data: FrameData::default(),
		animation_frame: "idle_3",
		duration: 6,
	},
	MoveData {
		data: FrameData::default(),
		animation_frame: "idle_4",
		duration: 3,
	},
];

pub fn idle_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in IDLE_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const FWALK_DATA: [MoveData; 6] = [
	MoveData {
		data: FrameData {
			speed: 6,
			..FrameData::default()
		},
		animation_frame: "fwalk_0",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: 6,
			..FrameData::default()
		},
		animation_frame: "fwalk_1",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: 6,
			..FrameData::default()
		},
		animation_frame: "fwalk_2",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: 6,
			..FrameData::default()
		},
		animation_frame: "fwalk_3",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: 6,
			..FrameData::default()
		},
		animation_frame: "fwalk_4",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: 6,
			..FrameData::default()
		},
		animation_frame: "fwalk_5",
		duration: 4,
	},
];

pub fn fwalk_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in FWALK_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const BWALK_DATA: [MoveData; 6] = [
	MoveData {
		data: FrameData {
			speed: -5,
			..FrameData::default()
		},
		animation_frame: "bwalk_0",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: -5,
			..FrameData::default()
		},
		animation_frame: "bwalk_1",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: -5,
			..FrameData::default()
		},
		animation_frame: "bwalk_2",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: -5,
			..FrameData::default()
		},
		animation_frame: "bwalk_3",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: -5,
			..FrameData::default()
		},
		animation_frame: "bwalk_4",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: -5,
			..FrameData::default()
		},
		animation_frame: "bwalk_5",
		duration: 4,
	},
];

pub fn bwalk_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in BWALK_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const NNORMAL_DATA: [MoveData; 6] = [
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "nnormal_0",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "nnormal_1",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			low: true,
			cancel: true,
			hitbox: Some(CBox {
				offsetx: 140,
				offsety: 0,
				x: 159,
				y: 46,
			}),
			hurtbox: [
				Some(CBox::base_hurtbox()),
				Some(CBox {
					offsetx: 149,
					offsety: 0,
					x: 175,
					y: 61,
				}),
			],
			..FrameData::default()
		},
		animation_frame: "nnormal_2",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			cancel: true,
			hurtbox: [
				Some(CBox::base_hurtbox()),
				Some(CBox {
					offsetx: 149,
					offsety: 0,
					x: 175,
					y: 61,
				}),
			],
			..FrameData::default()
		},
		animation_frame: "nnormal_2",
		duration: 10,
	},
	MoveData {
		data: FrameData {
			hurtbox: [
				Some(CBox::base_hurtbox()),
				Some(CBox {
					offsetx: 79,
					offsety: 0,
					x: 158,
					y: 74,
				}),
			],
			..FrameData::default()
		},
		animation_frame: "nnormal_3",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "nnormal_4",
		duration: 2,
	},
];

pub fn nnormal_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in NNORMAL_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const MNORMAL_DATA: [MoveData; 6] = [
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "mnormal_0",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "mnormal_1",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			cancel: true,
			hitbox: Some(CBox {
				offsetx: 130,
				offsety: 0,
				x: 130,
				y: 138,
			}),
			hurtbox: [
				Some(CBox::base_hurtbox()),
				Some(CBox {
					offsetx: 130,
					offsety: 0,
					x: 130,
					y: 138,
				}),
			],
			..FrameData::default()
		},
		animation_frame: "mnormal_2",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			cancel: true,
			hurtbox: [
				Some(CBox::base_hurtbox()),
				Some(CBox {
					offsetx: 130,
					offsety: 0,
					x: 130,
					y: 138,
				}),
			],
			..FrameData::default()
		},
		animation_frame: "mnormal_2",
		duration: 10,
	},
	MoveData {
		data: FrameData {
			hurtbox: [
				Some(CBox::base_hurtbox()),
				Some(CBox {
					offsetx: 111,
					offsety: 0,
					x: 111,
					y: 138,
				}),
			],
			..FrameData::default()
		},
		animation_frame: "mnormal_3",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "mnormal_4",
		duration: 2,
	},
];

pub fn mnormal_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in MNORMAL_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const NSPECIAL_DATA: [MoveData; 12] = [
	MoveData {
		data: FrameData {
			speed: 10,
			..FrameData::default()
		},
		animation_frame: "nspecial_0",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			speed: 13,
			..FrameData::default()
		},
		animation_frame: "nspecial_1",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			speed: 16,
			..FrameData::default()
		},
		animation_frame: "nspecial_2",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			speed: 16,
			..FrameData::default()
		},
		animation_frame: "nspecial_3",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			speed: 16,
			..FrameData::default()
		},
		animation_frame: "nspecial_4",
		duration: 1,
	},
	MoveData {
		data: FrameData {
			ender: true,
			speed: 16,
			hitbox: Some(CBox {
				offsetx: 158,
				offsety: 119,
				x: 158,
				y: 55,
			}),
			hurtbox: [
				Some(CBox::base_hurtbox()),
				Some(CBox {
					offsetx: 127,
					offsety: 120,
					x: 127,
					y: 73,
				}),
			],
			..FrameData::default()
		},
		animation_frame: "nspecial_5",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: 6,
			hurtbox: [
				Some(CBox::base_hurtbox()),
				Some(CBox {
					offsetx: 127,
					offsety: 120,
					x: 127,
					y: 73,
				}),
			],
			..FrameData::default()
		},
		animation_frame: "nspecial_5",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			speed: 3,
			hurtbox: [
				Some(CBox::base_hurtbox()),
				Some(CBox {
					offsetx: 127,
					offsety: 120,
					x: 127,
					y: 73,
				}),
			],
			..FrameData::default()
		},
		animation_frame: "nspecial_5",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			hurtbox: [
				Some(CBox::base_hurtbox()),
				Some(CBox {
					offsetx: 127,
					offsety: 120,
					x: 127,
					y: 73,
				}),
			],
			..FrameData::default()
		},
		animation_frame: "nspecial_5",
		duration: 7,
	},
	MoveData {
		data: FrameData {
			hurtbox: [
				Some(CBox::base_hurtbox()),
				Some(CBox {
					offsetx: 120,
					offsety: 120,
					x: 120,
					y: 73,
				}),
			],
			..FrameData::default()
		},
		animation_frame: "nspecial_6",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "nspecial_6",
		duration: 12,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "nspecial_7",
		duration: 2,
	},
];

pub fn nspecial_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in NSPECIAL_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const MSPECIAL_DATA: [MoveData; 11] = [
	MoveData {
		data: FrameData {
			speed: 8,
			hurtbox: [None, None],
			..FrameData::default()
		},
		animation_frame: "mspecial_0",
		duration: 1,
	},
	MoveData {
		data: FrameData {
			speed: 8,
			hurtbox: [None, None],
			..FrameData::default()
		},
		animation_frame: "mspecial_1",
		duration: 1,
	},
	MoveData {
		data: FrameData {
			ender: true,
			speed: 7,
			hitbox: Some(CBox {
				offsetx: 95,
				offsety: 0,
				x: 95,
				y: 158,
			}),
			hurtbox: [None, None],
			..FrameData::default()
		},
		animation_frame: "mspecial_2",
		duration: 1,
	},
	MoveData {
		data: FrameData {
			ender: true,
			speed: 5,
			hitbox: Some(CBox {
				offsetx: 95,
				offsety: 0,
				x: 95,
				y: 158,
			}),
			hurtbox: [None, None],
			..FrameData::default()
		},
		animation_frame: "mspecial_2",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			ender: true,
			speed: 5,
			hitbox: Some(CBox {
				offsetx: 95,
				offsety: 0,
				x: 95,
				y: 158,
			}),
			..FrameData::default()
		},
		animation_frame: "mspecial_3",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			speed: 5,
			..FrameData::default()
		},
		animation_frame: "mspecial_3",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			speed: 3,
			..FrameData::default()
		},
		animation_frame: "mspecial_3",
		duration: 5,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "mspecial_3",
		duration: 20,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "mspecial_4",
		duration: 10,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "mspecial_5",
		duration: 7,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "mspecial_6",
		duration: 2,
	},
];

pub fn mspecial_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in MSPECIAL_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const FDASH_DATA: [MoveData; 9] = [
	MoveData {
		data: FrameData {
			speed: 13,
			..FrameData::default()
		},
		animation_frame: "fdash_0",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			speed: 18,
			..FrameData::default()
		},
		animation_frame: "fdash_0",
		duration: 5,
	},
	MoveData {
		data: FrameData {
			speed: 18,
			..FrameData::default()
		},
		animation_frame: "fdash_1",
		duration: 1,
	},
	MoveData {
		data: FrameData {
			speed: 12,
			..FrameData::default()
		},
		animation_frame: "fdash_1",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			speed: 12,
			..FrameData::default()
		},
		animation_frame: "fdash_2",
		duration: 1,
	},
	MoveData {
		data: FrameData {
			speed: 5,
			..FrameData::default()
		},
		animation_frame: "fdash_2",
		duration: 1,
	},
	MoveData {
		data: FrameData {
			speed: 5,
			..FrameData::default()
		},
		animation_frame: "fdash_3",
		duration: 1,
	},
	MoveData {
		data: FrameData {
			speed: 3,
			..FrameData::default()
		},
		animation_frame: "fdash_3",
		duration: 1,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "fdash_4",
		duration: 1,
	},
];

pub fn fdash_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in FDASH_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const BDASH_DATA: [MoveData; 8] = [
	MoveData {
		data: FrameData {
			speed: -26,
			..FrameData::default()
		},
		animation_frame: "bdash_0",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			speed: -12,
			..FrameData::default()
		},
		animation_frame: "bdash_0",
		duration: 6,
	},
	MoveData {
		data: FrameData {
			speed: -8,
			..FrameData::default()
		},
		animation_frame: "bdash_0",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			speed: -8,
			..FrameData::default()
		},
		animation_frame: "bdash_1",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			speed: -3,
			..FrameData::default()
		},
		animation_frame: "bdash_1",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "bdash_1",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "bdash_2",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "bdash_3",
		duration: 1,
	},
];

pub fn bdash_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in BDASH_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const HIT_DATA: [MoveData; 5] = [
	MoveData {
		data: FrameData {
			speed: -9,
			..FrameData::default()
		},
		animation_frame: "hit_0",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: -6,
			..FrameData::default()
		},
		animation_frame: "hit_0",
		duration: 5,
	},
	MoveData {
		data: FrameData {
			speed: -2,
			..FrameData::default()
		},
		animation_frame: "hit_1",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "hit_2",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "hit_3",
		duration: 1,
	},
];

pub fn hit_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in HIT_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const HBLOCK_DATA: [MoveData; 5] = [
	MoveData {
		data: FrameData {
			speed: -8,
			..FrameData::default()
		},
		animation_frame: "hblock_0",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: -4,
			..FrameData::default()
		},
		animation_frame: "hblock_0",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			speed: -2,
			..FrameData::default()
		},
		animation_frame: "hblock_0",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "hblock_0",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "hblock_1",
		duration: 4,
	},
];

pub fn hblock_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in HBLOCK_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const LBLOCK_DATA: [MoveData; 5] = [
	MoveData {
		data: FrameData {
			speed: -8,
			..FrameData::default()
		},
		animation_frame: "lblock_0",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: -4,
			..FrameData::default()
		},
		animation_frame: "lblock_0",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			speed: -2,
			..FrameData::default()
		},
		animation_frame: "lblock_0",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "lblock_0",
		duration: 2,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "lblock_1",
		duration: 4,
	},
];

pub fn lblock_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in LBLOCK_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub const GUARD_BREAK_DATA: [MoveData; 5] = [
	MoveData {
		data: FrameData {
			speed: -8,
			hurtbox: [Some(CBox::guard_break_hurtbox()), None],
			..FrameData::default()
		},
		animation_frame: "guard_break_0",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			speed: -4,
			hurtbox: [Some(CBox::guard_break_hurtbox()), None],
			..FrameData::default()
		},
		animation_frame: "guard_break_0",
		duration: 3,
	},
	MoveData {
		data: FrameData {
			speed: -2,
			hurtbox: [Some(CBox::guard_break_hurtbox()), None],
			..FrameData::default()
		},
		animation_frame: "guard_break_0",
		duration: 4,
	},
	MoveData {
		data: FrameData {
			hurtbox: [Some(CBox::guard_break_hurtbox()), None],
			..FrameData::default()
		},
		animation_frame: "guard_break_0",
		duration: 20,
	},
	MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "guard_break_1",
		duration: 5,
	},
];

pub fn guard_break_data(frame: u8) -> Option<&'static MoveData> {
	let mut frame = (frame + 1) as usize;

	for d in GUARD_BREAK_DATA.iter() {
		frame = frame.saturating_sub(d.duration as usize);

		if frame == 0 {
			return Some(d);
		}
	}

	None
}

pub fn dead_data() -> &'static MoveData {
	const DATA: MoveData = MoveData {
		data: FrameData {
			..FrameData::default()
		},
		animation_frame: "dead_0",
		duration: 1,
	};

	&DATA
}

pub const fn move_length(data: &[MoveData]) -> u8 {
	let mut i = 0;
	let mut res = 0;

	// Not used `for` to be const compatible
	while i < data.len() {
		res += data[i].duration;
		i += 1;
	}

	res
}
