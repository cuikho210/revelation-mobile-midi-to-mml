use crate::{mml_event::MidiNoteState, mml_song::MmlSongOptions, pitch_class::PitchClass, utils};

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

        Self {
            midi_state,
            pitch_class,
            octave,
            velocity,
            position_in_smallest_unit,
            duration_in_smallest_unit,
            is_part_of_chord,
            mml_string: String::new(),
            mml_note_length: 0,
            song_options: options.to_owned(),
        }
    }

    pub fn update_mml_string(&mut self) {
        self.mml_string = utils::get_display_mml(
            self.duration_in_smallest_unit,
            &self.pitch_class,
            self.song_options.smallest_unit,
        );
        self.mml_note_length = utils::count_mml_notes(&self.mml_string);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MmlSongOptions, PitchClass, mml_event::MidiState};

    fn create_test_midi_note(
        key: u8,
        velocity: u8,
        position: usize,
        duration: usize,
    ) -> MidiNoteState {
        MidiNoteState {
            key,
            velocity,
            midi_state: MidiState {
                position_in_tick: position,
                duration_in_tick: duration,
                channel: 0,
            },
        }
    }

    #[test]
    fn test_mml_note_from_midi_state_basic() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Test middle C (MIDI key 60) quarter note
        let midi_note = create_test_midi_note(60, 64, 0, 480);
        let mut mml_note = MmlNote::from_midi_state(midi_note.clone(), &options, ppq, false);
        mml_note.update_mml_string();

        assert_eq!(mml_note.pitch_class, PitchClass::C);
        assert_eq!(mml_note.octave, 4); // Middle C is C4
        assert_eq!(mml_note.velocity, 7); // 64/127 * 15 ≈ 7.55 -> 7
        assert_eq!(mml_note.position_in_smallest_unit, 0);
        assert_eq!(mml_note.duration_in_smallest_unit, 16); // Quarter note = 16/64
        assert!(!mml_note.is_part_of_chord);
        assert_eq!(mml_note.mml_string, "c4");
        assert_eq!(mml_note.mml_note_length, 1);
    }

    #[test]
    fn test_mml_note_different_pitches() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Test all chromatic notes from C
        let test_cases = vec![
            (60, PitchClass::C),  // C
            (61, PitchClass::Db), // C#/Db
            (62, PitchClass::D),  // D
            (63, PitchClass::Eb), // D#/Eb
            (64, PitchClass::E),  // E
            (65, PitchClass::F),  // F
            (66, PitchClass::Gb), // F#/Gb
            (67, PitchClass::G),  // G
            (68, PitchClass::Ab), // G#/Ab
            (69, PitchClass::A),  // A
            (70, PitchClass::Bb), // A#/Bb
            (71, PitchClass::B),  // B
        ];

        for (midi_key, expected_pitch) in test_cases {
            let midi_note = create_test_midi_note(midi_key, 64, 0, 480);
            let mut mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
            mml_note.update_mml_string();
            assert_eq!(
                mml_note.pitch_class, expected_pitch,
                "Failed for MIDI key {}",
                midi_key
            );
        }
    }

    #[test]
    fn test_mml_note_octaves() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Test different octaves
        let test_cases = vec![
            (48, 3), // C3
            (60, 4), // C4 (Middle C)
            (72, 5), // C5
            (84, 6), // C6
        ];

        for (midi_key, expected_octave) in test_cases {
            let midi_note = create_test_midi_note(midi_key, 64, 0, 480);
            let mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
            assert_eq!(
                mml_note.octave, expected_octave,
                "Failed for MIDI key {}",
                midi_key
            );
        }
    }

    #[test]
    fn test_mml_note_velocity_conversion() {
        let options = MmlSongOptions::default(); // 0-15 range
        let ppq = 480;

        // Test velocity conversion
        let test_cases = vec![
            (0, 0),    // Minimum MIDI velocity -> minimum MML velocity
            (127, 15), // Maximum MIDI velocity -> maximum MML velocity
            (64, 7),   // Middle MIDI velocity -> middle MML velocity (64/127*15 ≈ 7.55 -> 7)
        ];

        for (midi_velocity, expected_mml_velocity) in test_cases {
            let midi_note = create_test_midi_note(60, midi_velocity, 0, 480);
            let mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
            assert_eq!(
                mml_note.velocity, expected_mml_velocity,
                "Failed for MIDI velocity {}",
                midi_velocity
            );
        }
    }

    #[test]
    fn test_mml_note_custom_velocity_range() {
        let mut options = MmlSongOptions::default();
        options.velocity_min = 5;
        options.velocity_max = 10;
        let ppq = 480;

        // Test with custom velocity range
        let midi_note = create_test_midi_note(60, 0, 0, 480);
        let mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
        assert_eq!(mml_note.velocity, 5); // Minimum should map to 5

        let midi_note = create_test_midi_note(60, 127, 0, 480);
        let mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
        assert_eq!(mml_note.velocity, 10); // Maximum should map to 10
    }

    #[test]
    fn test_mml_note_durations() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Test different note durations
        let test_cases = vec![
            (1920, 64, "c1"), // Whole note
            (960, 32, "c2"),  // Half note
            (480, 16, "c4"),  // Quarter note
            (240, 8, "c8"),   // Eighth note
            (120, 4, "c16"),  // Sixteenth note
        ];

        for (tick_duration, expected_units, expected_mml) in test_cases {
            let midi_note = create_test_midi_note(60, 64, 0, tick_duration);
            let mut mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
            mml_note.update_mml_string();
            assert_eq!(
                mml_note.duration_in_smallest_unit, expected_units,
                "Duration in units failed for {} ticks",
                tick_duration
            );
            assert_eq!(
                mml_note.mml_string, expected_mml,
                "MML string failed for {} ticks",
                tick_duration
            );
        }
    }

    #[test]
    fn test_mml_note_position_conversion() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Test position conversion from ticks to smallest units
        let test_cases = vec![
            (0, 0),    // Start of track
            (480, 16), // One quarter note in
            (960, 32), // One half note in
            (240, 8),  // One eighth note in
        ];

        for (tick_position, expected_units) in test_cases {
            let midi_note = create_test_midi_note(60, 64, tick_position, 480);
            let mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
            assert_eq!(
                mml_note.position_in_smallest_unit, expected_units,
                "Position conversion failed for {} ticks",
                tick_position
            );
        }
    }

    #[test]
    fn test_mml_note_chord_flag() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Test chord flag
        let midi_note = create_test_midi_note(60, 64, 0, 480);

        // Test as standalone note
        let mml_note = MmlNote::from_midi_state(midi_note.clone(), &options, ppq, false);
        assert!(!mml_note.is_part_of_chord);

        // Test as part of chord
        let mml_note_chord = MmlNote::from_midi_state(midi_note, &options, ppq, true);
        assert!(mml_note_chord.is_part_of_chord);
    }

    #[test]
    fn test_mml_note_update_mml_string() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        let midi_note = create_test_midi_note(60, 64, 0, 480);
        let mut mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
        mml_note.update_mml_string();

        // Initial state
        assert_eq!(mml_note.mml_string, "c4");
        assert_eq!(mml_note.mml_note_length, 1);

        // Modify duration and update
        mml_note.duration_in_smallest_unit = 24; // Dotted quarter
        mml_note.update_mml_string();

        assert_eq!(mml_note.mml_string, "c4.");
        assert_eq!(mml_note.mml_note_length, 1);

        // Test tied note
        mml_note.duration_in_smallest_unit = 20; // Quarter + sixteenth
        mml_note.update_mml_string();

        assert_eq!(mml_note.mml_string, "c4&c16");
        assert_eq!(mml_note.mml_note_length, 2);
    }

    #[test]
    fn test_mml_note_edge_cases() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Test very short duration
        let midi_note = create_test_midi_note(60, 64, 0, 30); // 1/64 note
        let mut mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
        mml_note.update_mml_string();
        assert_eq!(mml_note.duration_in_smallest_unit, 1);
        assert_eq!(mml_note.mml_string, "c64");

        // Test very long duration
        let midi_note = create_test_midi_note(60, 64, 0, 3840); // Two whole notes
        let mut mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
        mml_note.update_mml_string();
        assert_eq!(mml_note.duration_in_smallest_unit, 128);
        assert_eq!(mml_note.mml_string, "c1.&c2"); // Dotted whole + half

        // Test extreme velocity values
        let midi_note = create_test_midi_note(60, 0, 0, 480);
        let mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
        assert_eq!(mml_note.velocity, 0);

        let midi_note = create_test_midi_note(60, 127, 0, 480);
        let mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
        assert_eq!(mml_note.velocity, 15);
    }

    #[test]
    fn test_mml_note_rounding_behavior() {
        // Test that duration conversion handles rounding correctly
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Test edge cases around rounding
        let test_cases = vec![
            (29, 1), // Just under 1/64, should round to 1
            (31, 1), // Just over 1/64, should round to 1
            (44, 1), // Close to 1.5/64, should round to 1
            (46, 2), // Close to 1.5/64, should round to 2
        ];

        for (tick_duration, expected_units) in test_cases {
            let midi_note = create_test_midi_note(60, 64, 0, tick_duration);
            let mml_note = MmlNote::from_midi_state(midi_note, &options, ppq, false);
            assert_eq!(
                mml_note.duration_in_smallest_unit, expected_units,
                "Rounding failed for {} ticks",
                tick_duration
            );
        }
    }
}
