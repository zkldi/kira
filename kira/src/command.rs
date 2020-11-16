use crate::{
	instance::{InstanceId, InstanceSettings},
	parameter::ParameterId,
	sequence::{Sequence, SequenceId},
	sound::{Sound, SoundId},
	tempo::Tempo,
	track::effect::Effect,
	track::effect::EffectId,
	track::{effect::EffectSettings, SubTrackId, TrackIndex, TrackSettings},
	tween::Tween,
	value::Value,
};

#[derive(Debug)]
pub(crate) enum SoundCommand {
	LoadSound(SoundId, Sound),
	UnloadSound(SoundId),
}

#[derive(Debug, Clone)]
pub(crate) enum InstanceCommand {
	PlaySound(InstanceId, SoundId, Option<SequenceId>, InstanceSettings),
	SetInstanceVolume(InstanceId, Value),
	SetInstancePitch(InstanceId, Value),
	PauseInstance(InstanceId, Option<Tween>),
	ResumeInstance(InstanceId, Option<Tween>),
	StopInstance(InstanceId, Option<Tween>),
	PauseInstancesOfSound(SoundId, Option<Tween>),
	ResumeInstancesOfSound(SoundId, Option<Tween>),
	StopInstancesOfSound(SoundId, Option<Tween>),
	PauseInstancesOfSequence(SequenceId, Option<Tween>),
	ResumeInstancesOfSequence(SequenceId, Option<Tween>),
	StopInstancesOfSequence(SequenceId, Option<Tween>),
}

#[derive(Debug, Clone)]
pub(crate) enum MetronomeCommand {
	SetMetronomeTempo(Tempo),
	StartMetronome,
	PauseMetronome,
	StopMetronome,
}

#[derive(Debug, Clone)]
pub(crate) enum SequenceCommand<CustomEvent: Copy> {
	StartSequence(SequenceId, Sequence<CustomEvent>),
	MuteSequence(SequenceId),
	UnmuteSequence(SequenceId),
	PauseSequence(SequenceId),
	ResumeSequence(SequenceId),
	StopSequence(SequenceId),
}

pub(crate) enum MixerCommand {
	AddSubTrack(SubTrackId, TrackSettings),
	RemoveSubTrack(SubTrackId),
	AddEffect(TrackIndex, EffectId, Box<dyn Effect>, EffectSettings),
	RemoveEffect(EffectId),
}

#[derive(Debug, Clone)]
pub(crate) enum ParameterCommand {
	AddParameter(ParameterId, f64),
	RemoveParameter(ParameterId),
	SetParameter(ParameterId, f64, Option<Tween>),
}

pub(crate) enum Command<CustomEvent: Copy> {
	Sound(SoundCommand),
	Instance(InstanceCommand),
	Metronome(MetronomeCommand),
	Sequence(SequenceCommand<CustomEvent>),
	Mixer(MixerCommand),
	Parameter(ParameterCommand),
	EmitCustomEvent(CustomEvent),
}