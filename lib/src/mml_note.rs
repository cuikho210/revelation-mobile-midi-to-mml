use crate::{
    mml_event::MidiNoteState,
    mml_song::MmlSongOptions,
    pitch_class::PitchClass,
    utils,
};

#[derive(Debug, Clone)]
pub struct MmlNote {
    pub midi_state: MidiNoteState,
    pub pitch_class: PitchClass,
    pub octave: u8,
    pub velocity: u8,
    pub position_in_smallest_unit: usize,
    pub duration_in_smallest_unit: usize,
    pub is_part_of_chord: bool,
}

impl MmlNote {
    pub fn from_midi_state(
        midi_state: MidiNoteState,
        options: &MmlSongOptions,
        ppq: u16,
        is_part_of_chord: bool,
    ) -> Self {
        let (pitch_class, octave) = (
            utils::midi_key_to_pitch_class(midi_state.key),
            utils::midi_key_to_octave(midi_state.key),
        );

        let velocity = utils::midi_velocity_to_mml_velocity(
            midi_state.velocity,
            options.velocity_min,
            options.velocity_max,
        );

        let position_in_smallest_unit = utils::tick_to_smallest_unit(
            midi_state.midi_state.position_in_tick,
            ppq
        );

        let duration_in_smallest_unit = utils::tick_to_smallest_unit(
            midi_state.midi_state.duration_in_tick,
            ppq
        );

        Self {
            midi_state,
            pitch_class,
            octave,
            velocity,
            position_in_smallest_unit,
            duration_in_smallest_unit,
            is_part_of_chord,
        }
    }
}
