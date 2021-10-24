//! Pieces of audio that can be played multiple times.

mod handle;
pub mod instance;
mod seamless_loop;
pub mod static_sound;
pub mod streaming;
pub(crate) mod wrapper;

pub use handle::*;
pub use seamless_loop::*;

use std::time::Duration;

use atomic_arena::Index;

use crate::{loop_behavior::LoopBehavior, Frame};

/// A unique identifier for a sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundId(pub(crate) Index);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaybackInfo {
	pub position: f64,
	pub playback_rate: f64,
}

/// Represents a finite piece of audio.
#[allow(unused_variables)]
pub trait Sound: Send + Sync {
	/// Returns the duration of the sound.
	fn duration(&mut self) -> Duration;

	/// Returns the [`Frame`] that the sound should output
	/// at a given playback position.
	fn frame_at_position(&mut self, position: f64) -> Option<Frame>;

	/// Returns the suggested [`LoopBehavior`] of the sound,
	/// if any.
	fn default_loop_behavior(&mut self) -> Option<LoopBehavior> {
		None
	}

	fn report_playback_info(&mut self, playback_info: PlaybackInfo) {}
}
