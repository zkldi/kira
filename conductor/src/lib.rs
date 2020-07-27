mod backend;
mod sound;
pub mod sound_bank;
mod stereo_sample;

use backend::Backend;
use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Stream,
};
use sound_bank::SoundBank;
use std::error::Error;

pub enum SoundName {
	Test,
}

pub struct AudioManager {
	stream: Stream,
}

impl AudioManager {
	pub fn new(sound_bank: SoundBank) -> Result<Self, Box<dyn Error>> {
		let host = cpal::default_host();
		let device = host.default_output_device().unwrap();
		let mut supported_configs_range = device.supported_output_configs().unwrap();
		let supported_config = supported_configs_range
			.next()
			.unwrap()
			.with_max_sample_rate();
		let config = supported_config.config();
		let sample_rate = config.sample_rate.0;
		let channels = config.channels;
		let mut backend = Backend::new(sample_rate, sound_bank);
		let stream = device.build_output_stream(
			&config,
			move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
				for frame in data.chunks_exact_mut(channels as usize) {
					let out = backend.process();
					frame[0] = out.left;
					frame[1] = out.right;
				}
			},
			move |err| {},
		)?;
		stream.play()?;
		Ok(Self { stream: stream })
	}
}
