#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timer {
	Limited(u16),
	Unlimited,
}

impl Timer {
	pub const fn step(self) -> Self {
		let Timer::Limited(time) = self else {
			return self;
		};

		Timer::Limited(time - 1)
	}

	pub const fn seconds(self) -> u16 {
		let Timer::Limited(time) = self else {
			return 0;
		};

		let (div, mo) = (time / 60, time % 60);
		div + if mo > 0 { 1 } else { 0 }
	}

	pub const fn is_over(self) -> bool {
		matches!(self, Timer::Limited(0))
	}
}

#[cfg(test)]
mod test {
	use super::Timer;

	#[test]
	fn step() {
		let timer = Timer::Unlimited;
		assert_eq!(Timer::Unlimited, timer);
		timer.step();
		assert_eq!(Timer::Unlimited, timer);

		let timer = Timer::Limited(120);
		assert_eq!(Timer::Limited(120), timer);
		let timer = timer.step();
		assert_eq!(Timer::Limited(119), timer);
		let timer = timer.step();
		assert_eq!(Timer::Limited(118), timer);
		let timer = timer.step();
		assert_eq!(Timer::Limited(117), timer);
		let timer = timer.step();
		assert_eq!(Timer::Limited(116), timer);
		let timer = timer.step();
		assert_eq!(Timer::Limited(115), timer);
	}

	#[test]
	fn seconds() {
		let timer = Timer::Unlimited;
		assert_eq!(timer.seconds(), 0);
		timer.step();
		assert_eq!(timer.seconds(), 0);

		let timer = Timer::Limited(240);
		assert_eq!(timer.seconds(), 4);
		let timer = timer.step();
		assert_eq!(timer.seconds(), 4);

		let timer = Timer::Limited(62);
		assert_eq!(timer.seconds(), 2);
		let timer = timer.step();
		assert_eq!(timer.seconds(), 2);
		let timer = timer.step();
		assert_eq!(timer.seconds(), 1);
		let timer = timer.step();
		assert_eq!(timer.seconds(), 1);
	}

	#[test]
	fn is_over() {
		let timer = Timer::Unlimited;
		assert!(!timer.is_over());

		let timer = Timer::Limited(2);
		assert!(!timer.is_over());
		let timer = timer.step();
		assert!(!timer.is_over());
		let timer = timer.step();
		assert!(timer.is_over());
	}
}
