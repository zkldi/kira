//! Provides an interface for smoothly changing values over time.

mod mapping;
mod parameter;
mod parameters;
mod tween;

pub use mapping::Mapping;
pub(crate) use parameter::Parameter;
pub use parameter::ParameterId;
pub use parameters::Parameters;
pub use tween::{EaseDirection, Easing, Tween};