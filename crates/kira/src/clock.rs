//! Precise timing for audio events.

mod handle;
mod time;

#[cfg(test)]
mod test;

pub use handle::*;
pub use time::*;

use std::sync::{
	atomic::{AtomicBool, AtomicU64, Ordering},
	Arc,
};

use atomic_arena::Key;

use crate::{
	tween::{Tween, Tweener},
	ClockSpeed,
};

/// A unique identifier for a clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClockId(pub(crate) Key);

pub(crate) struct ClockShared {
	ticking: AtomicBool,
	ticks: AtomicU64,
	fractional_position: AtomicU64,
	removed: AtomicBool,
}

impl ClockShared {
	pub fn new() -> Self {
		Self {
			ticking: AtomicBool::new(false),
			ticks: AtomicU64::new(0),
			fractional_position: AtomicU64::new(0.0f64.to_bits()),
			removed: AtomicBool::new(false),
		}
	}

	pub fn ticking(&self) -> bool {
		self.ticking.load(Ordering::SeqCst)
	}

	pub fn ticks(&self) -> u64 {
		self.ticks.load(Ordering::SeqCst)
	}

	pub fn fractional_position(&self) -> f64 {
		f64::from_bits(self.fractional_position.load(Ordering::SeqCst))
	}

	pub fn is_marked_for_removal(&self) -> bool {
		self.removed.load(Ordering::SeqCst)
	}

	pub fn mark_for_removal(&self) {
		self.removed.store(true, Ordering::SeqCst);
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
	NotStarted,
	Started {
		ticks: u64,
		fractional_position: f64,
	},
}

pub(crate) struct Clock {
	shared: Arc<ClockShared>,
	ticking: bool,
	speed: Tweener<ClockSpeed>,
	state: State,
}

impl Clock {
	pub(crate) fn new(speed: ClockSpeed) -> Self {
		Self {
			shared: Arc::new(ClockShared::new()),
			ticking: false,
			speed: Tweener::new(speed),
			state: State::NotStarted,
		}
	}

	pub(crate) fn shared(&self) -> Arc<ClockShared> {
		self.shared.clone()
	}

	pub(crate) fn set_speed(&mut self, speed: ClockSpeed, tween: Tween) {
		self.speed.set(speed, tween);
	}

	pub(crate) fn start(&mut self) {
		self.ticking = true;
		self.shared.ticking.store(true, Ordering::SeqCst);
	}

	pub(crate) fn pause(&mut self) {
		self.ticking = false;
		self.shared.ticking.store(false, Ordering::SeqCst);
	}

	pub(crate) fn stop(&mut self) {
		self.pause();
		self.state = State::NotStarted;
		self.shared.ticks.store(0, Ordering::SeqCst);
	}

	pub(crate) fn on_start_processing(&mut self) {
		let (ticks, fractional_position) = match &self.state {
			State::NotStarted => (0, 0.0),
			State::Started {
				ticks,
				fractional_position,
			} => (*ticks, *fractional_position),
		};
		self.shared.ticks.store(ticks, Ordering::SeqCst);
		self.shared
			.fractional_position
			.store(fractional_position.to_bits(), Ordering::SeqCst);
	}

	/// Updates the [`Clock`].
	///
	/// If the tick count changes this update, returns `Some(tick_number)`.
	/// Otherwise, returns `None`.
	pub(crate) fn update(&mut self, dt: f64) -> Option<u64> {
		self.speed.update(dt);
		if !self.ticking {
			return None;
		}
		let mut new_tick_count = None;
		if self.state == State::NotStarted {
			self.state = State::Started {
				ticks: 0,
				fractional_position: 0.0,
			};
			new_tick_count = Some(0);
		}
		if let State::Started {
			ticks,
			fractional_position: tick_timer,
		} = &mut self.state
		{
			*tick_timer += self.speed.value().as_ticks_per_second() * dt;
			while *tick_timer >= 1.0 {
				*tick_timer -= 1.0;
				*ticks += 1;
				new_tick_count = Some(*ticks);
			}
		} else {
			panic!("clock state should be Started by now");
		}
		new_tick_count
	}

	pub(crate) fn on_clock_tick(&mut self, time: ClockTime) {
		self.speed.on_clock_tick(time);
	}
}
