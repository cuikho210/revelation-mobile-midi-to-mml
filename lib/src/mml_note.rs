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
    pub mml_string: String,
    pub mml_note_length: usize,
    pub song_options: MmlSongOptions,
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
            ppq,
            options.smallest_unit,
        );

        let duration_in_smallest_unit = utils::tick_to_smallest_unit(
            midi_state.midi_state.duration_in_tick,
            ppq,
            options.smallest_unit,
        );

        let mml_string = utils::get_display_mml(
            duration_in_smallest_unit,
            &pitch_class,
            options.smallest_unit,
        );

        let mml_note_length = utils::count_mml_note(&mml_string);

        Self {
            midi_state,
            pitch_class,
            octave,
            velocity,
            position_in_smallest_unit,
            duration_in_smallest_unit,
            is_part_of_chord,
            mml_string,
            mml_note_length,
            song_options: options.to_owned(), 
        }
    }

    pub fn update_mml_string(&mut self, smallest_unit: usize) {
        self.mml_string = utils::get_display_mml(self.duration_in_smallest_unit, &self.pitch_class, smallest_unit);
        self.mml_note_length = utils::count_mml_note(&self.mml_string);
    }
}
