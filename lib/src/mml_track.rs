use std::collections::HashMap;

use rayon::prelude::*;

use crate::{
    Instrument,
    mml_event::{BridgeEvent, MidiNoteState, MmlEvent},
    mml_song::MmlSongOptions,
    parser::bridge_events_to_mml_events,
    utils,
};

#[derive(Debug, Clone)]
pub struct MmlTrack {
    pub name: String,
    pub instrument: Instrument,
    pub events: Vec<MmlEvent>,
    pub song_options: MmlSongOptions,
    pub bridge_meta_events: Vec<BridgeEvent>,
    pub bridge_note_events: Vec<BridgeEvent>,
    pub bridge_events: Vec<BridgeEvent>,
    pub ppq: u16,
    pub mml_note_length: usize,
}

impl MmlTrack {
    pub fn from_bridge_events(
        name: String,
        bridge_meta_events: Vec<BridgeEvent>,
        bridge_note_events: Vec<BridgeEvent>,
        song_options: MmlSongOptions,
        ppq: u16,
    ) -> Self {
        let mut mml_track = Self {
            name,
            events: Vec::new(),
            instrument: Instrument::default(),
            bridge_meta_events,
            bridge_note_events,
            bridge_events: Vec::new(),
            song_options,
            ppq,
            mml_note_length: 0,
        };

        mml_track.generate_mml_events();
        mml_track
    }

    pub fn apply_keymap(&mut self, keymap: &HashMap<u8, u8>) {
        self.events
            .par_chunks_mut(num_cpus::get())
            .for_each(|events| {
                for e in events.iter_mut() {
                    if let MmlEvent::Note(note) = e
                        && let Some(new_midi_key) = keymap.get(&note.midi_state.key)
                    {
                        note.apply_keymap(*new_midi_key, self.song_options.smallest_unit);
                    }
                }
            });
    }

    pub fn split(&self) -> (Self, Self) {
        let (mut track_a, mut track_b) = self.split_track_by_override();

        if self.song_options.auto_equalize_note_length {
            let is_out_of_range = track_a.mml_note_length > 3000 || track_b.mml_note_length > 3000;

            let is_too_different = {
                let ratio = 0.5_f32;
                let diff_a =
                    track_a.mml_note_length > (track_b.mml_note_length as f32 * ratio) as usize;
                let diff_b =
                    track_b.mml_note_length > (track_a.mml_note_length as f32 * ratio) as usize;
                diff_a || diff_b
            };

            if is_out_of_range || is_too_different {
                utils::equalize_tracks(&mut track_a, &mut track_b);
            }
        }

        track_a.instrument = self.instrument.to_owned();
        track_b.instrument = self.instrument.to_owned();

        (track_a, track_b)
    }

    pub fn merge(&mut self, other: &mut Self) {
        self.bridge_note_events
            .append(&mut other.bridge_note_events);
        self.bridge_note_events.sort();

        self.name = format!("{}+{}", &self.name, other.name);
        self.generate_mml_events();
    }

    pub fn to_mml(&self) -> String {
        let mut mml = String::new();

        for event in self.events.iter() {
            mml.push_str(&event.to_mml(self.song_options.smallest_unit));
        }

        mml
    }

    // TODO: Whitespace might cause errors in the game
    // pub fn to_mml_debug(&self) -> String {
    //     let mut mml = String::new();
    //     let mut notes_on_row: usize = 0;
    //
    //     for event in self.events.iter() {
    //         let current_mml = &event.to_mml_debug(self.song_options.smallest_unit);
    //         mml.push_str(&current_mml);
    //
    //         notes_on_row += event.get_duration();
    //         if notes_on_row >= 64 {
    //             notes_on_row = 0;
    //             mml.push('\n');
    //         }
    //     }
    //
    //     mml
    // }

    pub fn apply_boot_velocity(&mut self, velocity_diff: u8) {
        if velocity_diff > 0 {
            for event in self.events.iter_mut() {
                if let MmlEvent::Velocity(velocity) = event {
                    *velocity += velocity_diff
                }
            }
        }
    }

    pub fn generate_mml_events(&mut self) {
        self.apply_meta_events();

        let (events, instrument) =
            bridge_events_to_mml_events(&self.bridge_events, &self.song_options, self.ppq);

        if let Some(instrument) = instrument {
            self.instrument = instrument;
        }

        self.events = events;
        self.update_mml_note_length();
    }

    fn apply_meta_events(&mut self) {
        self.bridge_events =
            Vec::with_capacity(self.bridge_meta_events.len() + self.bridge_note_events.len());
        self.bridge_events
            .extend(self.bridge_note_events.to_owned());
        self.bridge_events
            .extend(self.bridge_meta_events.to_owned());
        self.bridge_events.sort();
    }

    fn update_mml_note_length(&mut self) {
        let mut note_length = 0usize;

        for event in self.events.iter() {
            if let MmlEvent::Note(note) = event {
                note_length += note.mml_note_length;
            }
        }

        self.mml_note_length = note_length;
    }

    fn split_track_by_override(&self) -> (Self, Self) {
        let mut max_end_position = 0usize;
        let mut before_note: Option<MidiNoteState> = None;
        let mut bridges_a: Vec<BridgeEvent> = Vec::new();
        let mut bridges_b: Vec<BridgeEvent> = Vec::new();

        for i in 0..self.bridge_note_events.len() {
            let current_bridge_event_ref = self.bridge_note_events.get(i).unwrap();
            let current_bridge_event = current_bridge_event_ref.to_owned();

            if let BridgeEvent::Note(current_note) = current_bridge_event_ref {
                let current_end_position = current_note.midi_state.position_in_tick
                    + current_note.midi_state.duration_in_tick;

                if current_end_position > max_end_position {
                    max_end_position = current_end_position;
                }

                if let Some(before_note) = before_note {
                    let note_pos_isize = current_note.midi_state.position_in_tick as isize;
                    let before_note_pos_isize = before_note.midi_state.position_in_tick as isize;
                    let start_pos_diff = note_pos_isize - before_note_pos_isize;
                    let min_gap_for_chord_isize = self.song_options.min_gap_for_chord as isize;
                    let min_gap_for_chord_in_smallest_unit =
                        min_gap_for_chord_isize * self.song_options.smallest_unit as isize;

                    if start_pos_diff <= min_gap_for_chord_in_smallest_unit {
                        bridges_a.push(current_bridge_event);
                    } else if current_note.midi_state.position_in_tick < max_end_position {
                        bridges_b.push(current_bridge_event);
                    } else {
                        bridges_a.push(current_bridge_event);
                    }
                } else {
                    bridges_a.push(current_bridge_event);
                }

                before_note = Some(current_note.to_owned());
            }
        }

        let track_a = Self::from_bridge_events(
            format!("{}.0", self.name),
            self.bridge_meta_events.to_owned(),
            bridges_a,
            self.song_options.to_owned(),
            self.ppq,
        );

        let track_b = Self::from_bridge_events(
            format!("{}.1", self.name),
            self.bridge_meta_events.to_owned(),
            bridges_b,
            self.song_options.to_owned(),
            self.ppq,
        );

        (track_a, track_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MmlSongOptions,
        mml_event::{BridgeEvent, MidiNoteState, MidiState, MmlEvent},
    };

    fn create_test_midi_note_state(
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
    fn test_mml_track_from_bridge_events_basic() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        let midi_note = create_test_midi_note_state(60, 64, 0, 480); // C4 quarter note
        let bridge_note_events = vec![BridgeEvent::Note(midi_note)];
        let bridge_meta_events = vec![];

        let track = MmlTrack::from_bridge_events(
            "test_track".to_string(),
            bridge_meta_events,
            bridge_note_events,
            options,
            ppq,
        );

        assert_eq!(track.name, "test_track");
        assert_eq!(track.events.len(), 3); // Velocity, Octave, Note
        assert_eq!(track.mml_note_length, 1);

        // Check that it contains a note event
        let has_note = track.events.iter().any(|e| matches!(e, MmlEvent::Note(_)));
        assert!(has_note);
    }

    #[test]
    fn test_mml_track_to_mml() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        let midi_note = create_test_midi_note_state(60, 64, 0, 480);
        let bridge_note_events = vec![BridgeEvent::Note(midi_note)];
        let bridge_meta_events = vec![];

        let track = MmlTrack::from_bridge_events(
            "test".to_string(),
            bridge_meta_events,
            bridge_note_events,
            options,
            ppq,
        );

        let mml_string = track.to_mml();

        // Should contain velocity, octave, and note
        assert!(mml_string.contains("v"));
        assert!(mml_string.contains("o"));
        assert!(mml_string.contains("c4"));
    }

    #[test]
    fn test_mml_track_merge() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Create first track with C note
        let midi_note1 = create_test_midi_note_state(60, 64, 0, 480);
        let bridge_events1 = vec![BridgeEvent::Note(midi_note1)];
        let mut track1 = MmlTrack::from_bridge_events(
            "track1".to_string(),
            vec![],
            bridge_events1,
            options.clone(),
            ppq,
        );

        // Create second track with D note at different time
        let midi_note2 = create_test_midi_note_state(62, 64, 480, 480);
        let bridge_events2 = vec![BridgeEvent::Note(midi_note2)];
        let mut track2 = MmlTrack::from_bridge_events(
            "track2".to_string(),
            vec![],
            bridge_events2,
            options,
            ppq,
        );

        let original_events_count = track1.events.len();
        track1.merge(&mut track2);

        // Name should be combined
        assert_eq!(track1.name, "track1+track2");

        // Should have more bridge events (but events are regenerated)
        assert_eq!(track1.bridge_note_events.len(), 2);

        // Should have regenerated MML events (could be different count due to merging logic)
        assert!(track1.events.len() >= original_events_count);
    }

    #[test]
    fn test_mml_track_split() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Create track with multiple notes
        let midi_note1 = create_test_midi_note_state(60, 64, 0, 480);
        let midi_note2 = create_test_midi_note_state(62, 64, 480, 480);
        let midi_note3 = create_test_midi_note_state(64, 64, 960, 480);

        let bridge_events = vec![
            BridgeEvent::Note(midi_note1),
            BridgeEvent::Note(midi_note2),
            BridgeEvent::Note(midi_note3),
        ];

        let track = MmlTrack::from_bridge_events(
            "original".to_string(),
            vec![],
            bridge_events,
            options,
            ppq,
        );

        let (track_a, track_b) = track.split();

        // Names should be suffixed
        assert_eq!(track_a.name, "original.0");
        assert_eq!(track_b.name, "original.1");

        // Both tracks should have some notes
        let total_bridge_events =
            track_a.bridge_note_events.len() + track_b.bridge_note_events.len();
        assert_eq!(total_bridge_events, 3); // All original notes should be distributed

        // Both should have some events
        assert!(!track_a.events.is_empty());
        // track_b might be empty if all notes went to track_a due to split logic
    }

    #[test]
    fn test_mml_track_apply_boot_velocity() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        let midi_note = create_test_midi_note_state(60, 64, 0, 480);
        let bridge_events = vec![BridgeEvent::Note(midi_note)];
        let mut track =
            MmlTrack::from_bridge_events("test".to_string(), vec![], bridge_events, options, ppq);

        // Find original velocity
        let original_velocity = track
            .events
            .iter()
            .find_map(|e| match e {
                MmlEvent::Velocity(v) => Some(*v),
                _ => None,
            })
            .expect("Should have velocity event");

        // Apply boot velocity
        track.apply_boot_velocity(3);

        // Find new velocity
        let new_velocity = track
            .events
            .iter()
            .find_map(|e| match e {
                MmlEvent::Velocity(v) => Some(*v),
                _ => None,
            })
            .expect("Should have velocity event");

        assert_eq!(new_velocity, original_velocity + 3);
    }

    #[test]
    fn test_mml_track_with_tempo_events() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        let tempo_event = BridgeEvent::Tempo(
            120,
            MidiState {
                position_in_tick: 0,
                duration_in_tick: 0,
                channel: 0,
            },
        );

        let midi_note = create_test_midi_note_state(60, 64, 0, 480);
        let note_event = BridgeEvent::Note(midi_note);

        let bridge_meta_events = vec![tempo_event];
        let bridge_note_events = vec![note_event];

        let track = MmlTrack::from_bridge_events(
            "tempo_test".to_string(),
            bridge_meta_events,
            bridge_note_events,
            options,
            ppq,
        );

        // Should have tempo event in the results
        let has_tempo = track
            .events
            .iter()
            .any(|e| matches!(e, MmlEvent::Tempo(_, _)));
        assert!(has_tempo);

        let mml = track.to_mml();
        assert!(mml.contains("t120"));
    }

    #[test]
    fn test_mml_track_duration_calculation() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Create notes with known durations
        let midi_note1 = create_test_midi_note_state(60, 64, 0, 480); // Quarter note
        let midi_note2 = create_test_midi_note_state(62, 64, 480, 240); // Eighth note

        let bridge_events = vec![BridgeEvent::Note(midi_note1), BridgeEvent::Note(midi_note2)];

        let track = MmlTrack::from_bridge_events(
            "duration_test".to_string(),
            vec![],
            bridge_events,
            options,
            ppq,
        );

        // mml_note_length should be calculated correctly
        // Each note contributes to the total MML note length
        assert!(track.mml_note_length > 0);

        // Verify the calculation by checking individual notes
        let mut expected_length = 0;
        for event in &track.events {
            if let MmlEvent::Note(note) = event {
                expected_length += note.mml_note_length;
            }
        }
        assert_eq!(track.mml_note_length, expected_length);
    }

    #[test]
    fn test_mml_track_empty_events() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Create track with no notes
        let track = MmlTrack::from_bridge_events("empty".to_string(), vec![], vec![], options, ppq);

        assert_eq!(track.events.len(), 0);
        assert_eq!(track.mml_note_length, 0);
        assert_eq!(track.to_mml(), "");
    }

    #[test]
    fn test_mml_track_bridge_events_sorting() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Create events out of order
        let midi_note1 = create_test_midi_note_state(60, 64, 480, 240); // Later note
        let midi_note2 = create_test_midi_note_state(62, 64, 0, 240); // Earlier note

        let bridge_events = vec![BridgeEvent::Note(midi_note1), BridgeEvent::Note(midi_note2)];

        let track = MmlTrack::from_bridge_events(
            "sorting_test".to_string(),
            vec![],
            bridge_events,
            options,
            ppq,
        );

        // Verify that events are processed in time order
        // The track should handle the sorting internally
        assert_eq!(track.bridge_note_events.len(), 2);

        // Check that events are sorted by position
        for i in 1..track.bridge_events.len() {
            let prev_pos = match &track.bridge_events[i - 1] {
                BridgeEvent::Note(note) => note.midi_state.position_in_tick,
                BridgeEvent::Tempo(_, state) => state.position_in_tick,
                BridgeEvent::ProgramChange(_, state) => state.position_in_tick,
            };

            let curr_pos = match &track.bridge_events[i] {
                BridgeEvent::Note(note) => note.midi_state.position_in_tick,
                BridgeEvent::Tempo(_, state) => state.position_in_tick,
                BridgeEvent::ProgramChange(_, state) => state.position_in_tick,
            };

            assert!(curr_pos >= prev_pos, "Events should be sorted by position");
        }
    }
}
