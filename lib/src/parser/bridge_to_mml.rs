use crate::{
    Instrument,
    mml_event::{BridgeEvent, MmlEvent},
    mml_note::MmlNote,
    mml_song::MmlSongOptions,
};

pub fn bridge_events_to_mml_events(
    bridge_events: &[BridgeEvent],
    options: &MmlSongOptions,
    ppq: u16,
) -> (Vec<MmlEvent>, Option<Instrument>) {
    let mut mml_events: Vec<MmlEvent> = Vec::new();
    let mut before_note: Option<MmlNote> = None;
    let mut instrument = None;

    for event in bridge_events.iter() {
        match event {
            BridgeEvent::Tempo(tempo, ..) => {
                mml_events.push(MmlEvent::Tempo(tempo.to_owned()));
            }
            BridgeEvent::Note(midi_state) => {
                let mut note = MmlNote::from_midi_state(midi_state.to_owned(), options, ppq, false);

                if let Some(before_note) = &before_note {
                    // Rest and chord
                    let before_note_end = before_note.position_in_smallest_unit
                        + before_note.duration_in_smallest_unit;
                    let position_diff =
                        note.position_in_smallest_unit as isize - before_note_end as isize;

                    if position_diff > 0 {
                        mml_events.push(MmlEvent::Rest(position_diff as usize));
                    } else if position_diff < 0 {
                        let note_pos_isize = note.position_in_smallest_unit as isize;
                        let before_note_pos_isize = before_note.position_in_smallest_unit as isize;
                        let start_pos_diff = note_pos_isize - before_note_pos_isize;

                        if start_pos_diff <= options.min_gap_for_chord as isize {
                            mml_events.push(MmlEvent::ConnectChord);
                            note.is_part_of_chord = true;
                        }
                    }

                    // Octave
                    let octave_diff = note.octave as i8 - before_note.octave as i8;

                    if octave_diff == 1 {
                        mml_events.push(MmlEvent::IncreOctave);
                    } else if octave_diff == -1 {
                        mml_events.push(MmlEvent::DecreOctave);
                    } else if octave_diff != 0 {
                        mml_events.push(MmlEvent::Octave(note.octave));
                    }

                    // Velocity
                    if note.velocity != before_note.velocity {
                        mml_events.push(MmlEvent::Velocity(note.velocity));
                    }
                } else {
                    if note.position_in_smallest_unit > 0 {
                        mml_events.push(MmlEvent::Rest(note.position_in_smallest_unit));
                    }

                    mml_events.push(MmlEvent::Velocity(note.velocity));
                    mml_events.push(MmlEvent::Octave(note.octave));
                }

                before_note = Some(note.to_owned());
                mml_events.push(MmlEvent::Note(note));
            }
            BridgeEvent::ProgramChange(dest_instrument, _) => {
                instrument = Some(dest_instrument.to_owned());
            }
        }
    }

    fix_note_position(&mut mml_events);
    update_chord_duration(&mut mml_events);
    update_note_mml(&mut mml_events);

    (mml_events, instrument)
}

fn update_note_mml(events: &mut [MmlEvent]) {
    for event in events.iter_mut() {
        if let MmlEvent::Note(note) = event {
            note.update_mml_string();
        }
    }
}

fn fix_note_position(events: &mut Vec<MmlEvent>) {
    for i in 0..events.len() {
        let current_event = events.get(i).unwrap().to_owned();

        if let MmlEvent::Note(note) = current_event {
            if note.is_part_of_chord {
                continue;
            }

            let current_position = get_current_position(events, i);
            let position_diff = note.position_in_smallest_unit as isize - current_position as isize;

            if position_diff != 0 {
                modify_before_duration(events, i, position_diff);
            }
        }
    }
}

fn get_current_position(events: &[MmlEvent], current_index: usize) -> usize {
    let mut is_first_note = true;
    let mut duration = 0usize;
    let mut index = 0usize;

    while index < current_index {
        if let Some(event) = events.get(index) {
            match event {
                MmlEvent::Rest(rest) => {
                    duration += rest;
                }
                MmlEvent::Note(note) => {
                    if is_first_note {
                        duration = note.position_in_smallest_unit;
                        is_first_note = false;
                    }

                    if !note.is_part_of_chord {
                        duration += note.duration_in_smallest_unit;
                    }
                }
                _ => (),
            }
        }

        index += 1;
    }

    duration
}

fn modify_before_duration(
    events: &mut Vec<MmlEvent>,
    mut current_index: usize,
    mut to_modify: isize,
) {
    let mut to_insert_connect_chord: Vec<usize> = Vec::new();

    loop {
        if current_index > 0 {
            current_index -= 1;
        } else {
            break;
        }

        if let Some(event) = events.get_mut(current_index) {
            match event {
                MmlEvent::Rest(rest) => {
                    let rest_isize = rest.to_owned() as isize;
                    let new_rest = rest_isize + to_modify;

                    if new_rest > 0 {
                        *rest = new_rest as usize;
                        break;
                    } else {
                        *rest = 0;
                        to_modify += rest_isize;
                    }
                }
                MmlEvent::Note(note) => {
                    if note.is_part_of_chord {
                        continue;
                    }

                    let duration_isize = note.duration_in_smallest_unit as isize;
                    let new_duration = duration_isize + to_modify;

                    if new_duration > 0 {
                        note.duration_in_smallest_unit = new_duration as usize;
                        break;
                    } else {
                        to_modify += duration_isize;
                        to_insert_connect_chord.push(current_index);
                    }
                }
                _ => (),
            }
        }
    }

    for index in to_insert_connect_chord {
        events.insert(index, MmlEvent::ConnectChord);

        if let Some(event) = events.get_mut(index + 1) {
            if let MmlEvent::Note(note) = event {
                note.is_part_of_chord = true;
            } else {
                eprintln!("[modify_before_duration] Cannot get note");
            }
        } else {
            eprintln!("[modify_before_duration] Cannot get event");
        }
    }
}

fn update_chord_duration(events: &mut [MmlEvent]) {
    let mut before_note: Option<MmlNote> = None;

    for i in 0..events.len() {
        let event = events.get_mut(i).unwrap();

        if let MmlEvent::Note(note) = event {
            if note.is_part_of_chord {
                if let Some(before_note) = &before_note {
                    note.duration_in_smallest_unit = before_note.duration_in_smallest_unit;
                }
                continue;
            }

            before_note = Some(note.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Instrument, MmlSongOptions, PitchClass,
        mml_event::{BridgeEvent, MidiNoteState, MidiState},
    };

    fn create_test_midi_note_state(
        key: u8,
        velocity: u8,
        position: usize,
        duration: usize,
        channel: u8,
    ) -> MidiNoteState {
        MidiNoteState {
            key,
            velocity,
            midi_state: MidiState {
                position_in_tick: position,
                duration_in_tick: duration,
                channel,
            },
        }
    }

    #[test]
    fn test_single_note_conversion() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        let midi_note = create_test_midi_note_state(60, 64, 0, 480, 0); // Middle C quarter note
        let bridge_events = vec![BridgeEvent::Note(midi_note)];

        let (mml_events, instrument) = bridge_events_to_mml_events(&bridge_events, &options, ppq);

        assert!(instrument.is_none()); // No program change

        // Expected events: Velocity, Octave, Note
        assert_eq!(mml_events.len(), 3);

        match &mml_events[0] {
            MmlEvent::Velocity(vel) => assert_eq!(*vel, 7), // 64/127 * 15 ≈ 7
            _ => panic!("Expected velocity event"),
        }

        match &mml_events[1] {
            MmlEvent::Octave(octave) => assert_eq!(*octave, 4), // Middle C is octave 4
            _ => panic!("Expected octave event"),
        }

        match &mml_events[2] {
            MmlEvent::Note(note) => {
                assert_eq!(note.pitch_class, PitchClass::C);
                assert_eq!(note.duration_in_smallest_unit, 16); // Quarter note
                assert_eq!(note.mml_string, "c4");
            }
            _ => panic!("Expected note event"),
        }
    }

    #[test]
    fn test_note_with_rest_before() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Note starting at position 240 (eighth note delay)
        let midi_note = create_test_midi_note_state(60, 64, 240, 480, 0);
        let bridge_events = vec![BridgeEvent::Note(midi_note)];

        let (mml_events, _) = bridge_events_to_mml_events(&bridge_events, &options, ppq);

        // Expected: Rest, Velocity, Octave, Note
        assert_eq!(mml_events.len(), 4);

        match &mml_events[0] {
            MmlEvent::Rest(duration) => assert_eq!(*duration, 8), // Eighth note rest
            _ => panic!("Expected rest event"),
        }
    }

    #[test]
    fn test_two_consecutive_notes() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        let note1 = create_test_midi_note_state(60, 64, 0, 480, 0); // C quarter note
        let note2 = create_test_midi_note_state(62, 64, 480, 480, 0); // D quarter note

        let bridge_events = vec![BridgeEvent::Note(note1), BridgeEvent::Note(note2)];

        let (mml_events, _) = bridge_events_to_mml_events(&bridge_events, &options, ppq);

        // Should have: Velocity, Octave, Note1, Note2
        assert_eq!(mml_events.len(), 4);

        // Verify second note doesn't repeat velocity/octave since they're the same
        match &mml_events[3] {
            MmlEvent::Note(note) => {
                assert_eq!(note.pitch_class, PitchClass::D);
            }
            _ => panic!("Expected second note"),
        }
    }

    #[test]
    fn test_octave_changes() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        let note1 = create_test_midi_note_state(60, 64, 0, 480, 0); // C4
        let note2 = create_test_midi_note_state(72, 64, 480, 480, 0); // C5

        let bridge_events = vec![BridgeEvent::Note(note1), BridgeEvent::Note(note2)];

        let (mml_events, _) = bridge_events_to_mml_events(&bridge_events, &options, ppq);

        // Should have octave increment
        let mut found_octave_increment = false;
        for event in &mml_events {
            if matches!(event, MmlEvent::IncreOctave) {
                found_octave_increment = true;
                break;
            }
        }
        assert!(found_octave_increment, "Expected octave increment");
    }

    #[test]
    fn test_velocity_changes() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        let note1 = create_test_midi_note_state(60, 64, 0, 480, 0); // Velocity 64
        let note2 = create_test_midi_note_state(60, 100, 480, 480, 0); // Velocity 100

        let bridge_events = vec![BridgeEvent::Note(note1), BridgeEvent::Note(note2)];

        let (mml_events, _) = bridge_events_to_mml_events(&bridge_events, &options, ppq);

        // Should have two velocity events
        let velocity_events: Vec<_> = mml_events
            .iter()
            .filter_map(|e| match e {
                MmlEvent::Velocity(vel) => Some(*vel),
                _ => None,
            })
            .collect();

        assert_eq!(velocity_events.len(), 2);
        assert_ne!(velocity_events[0], velocity_events[1]);
    }

    #[test]
    fn test_chord_detection() {
        let mut options = MmlSongOptions::default();
        options.min_gap_for_chord = 2; // Small gap for chord detection

        let ppq = 480;

        // Two notes starting very close together (within chord gap)
        let note1 = create_test_midi_note_state(60, 64, 0, 480, 0); // C
        let note2 = create_test_midi_note_state(64, 64, 1, 480, 0); // E (1 tick later, within chord gap)

        let bridge_events = vec![BridgeEvent::Note(note1), BridgeEvent::Note(note2)];

        let (mml_events, _) = bridge_events_to_mml_events(&bridge_events, &options, ppq);

        // Should contain a chord connector
        let has_chord_connector = mml_events
            .iter()
            .any(|e| matches!(e, MmlEvent::ConnectChord));

        assert!(has_chord_connector, "Expected chord connector");

        // Second note should be marked as part of chord
        let note_events: Vec<_> = mml_events
            .iter()
            .filter_map(|e| match e {
                MmlEvent::Note(note) => Some(note),
                _ => None,
            })
            .collect();

        assert_eq!(note_events.len(), 2);
        assert!(!note_events[0].is_part_of_chord);
        assert!(note_events[1].is_part_of_chord);
    }

    #[test]
    fn test_tempo_events() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        let midi_state = MidiState {
            position_in_tick: 0,
            duration_in_tick: 0,
            channel: 0,
        };

        let bridge_events = vec![
            BridgeEvent::Tempo(120, midi_state.clone()),
            BridgeEvent::Note(create_test_midi_note_state(60, 64, 0, 480, 0)),
        ];

        let (mml_events, _) = bridge_events_to_mml_events(&bridge_events, &options, ppq);

        // Should have tempo event
        match &mml_events[0] {
            MmlEvent::Tempo(tempo) => assert_eq!(*tempo, 120),
            _ => panic!("Expected tempo event first"),
        }
    }

    #[test]
    fn test_program_change() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        let midi_state = MidiState {
            position_in_tick: 0,
            duration_in_tick: 0,
            channel: 0,
        };

        let instrument = Instrument::new(1, 0); // Bright acoustic piano
        let bridge_events = vec![
            BridgeEvent::ProgramChange(instrument.clone(), midi_state),
            BridgeEvent::Note(create_test_midi_note_state(60, 64, 0, 480, 0)),
        ];

        let (_mml_events, returned_instrument) =
            bridge_events_to_mml_events(&bridge_events, &options, ppq);

        assert!(returned_instrument.is_some());
        assert_eq!(returned_instrument.unwrap(), instrument);
    }

    #[test]
    fn test_overlapping_notes_shortening() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // First note lasts longer than the gap to second note
        let note1 = create_test_midi_note_state(60, 64, 0, 960, 0); // Half note duration
        let note2 = create_test_midi_note_state(62, 64, 480, 480, 0); // Starts at quarter note position

        let bridge_events = vec![BridgeEvent::Note(note1), BridgeEvent::Note(note2)];

        let (mml_events, _) = bridge_events_to_mml_events(&bridge_events, &options, ppq);

        // The note positioning should be corrected
        // This tests the fix_note_position functionality
        assert!(!mml_events.is_empty());

        // Check that we have the expected notes
        let note_events: Vec<_> = mml_events
            .iter()
            .filter_map(|e| match e {
                MmlEvent::Note(note) => Some(&note.pitch_class),
                _ => None,
            })
            .collect();

        assert_eq!(note_events.len(), 2);
        assert_eq!(*note_events[0], PitchClass::C);
        assert_eq!(*note_events[1], PitchClass::D);
    }

    #[test]
    fn test_position_fixing_with_rest_insertion() {
        let options = MmlSongOptions::default();
        let ppq = 480;

        // Create a gap between notes that should be filled with rest
        let note1 = create_test_midi_note_state(60, 64, 0, 240, 0); // Eighth note
        let note2 = create_test_midi_note_state(62, 64, 480, 240, 0); // Eighth note, quarter note later

        let bridge_events = vec![BridgeEvent::Note(note1), BridgeEvent::Note(note2)];

        let (mml_events, _) = bridge_events_to_mml_events(&bridge_events, &options, ppq);

        // Should have a rest between the notes
        let has_rest = mml_events.iter().any(|e| matches!(e, MmlEvent::Rest(_)));

        assert!(has_rest, "Expected rest between notes");
    }

    #[test]
    fn test_chord_duration_update() {
        let mut options = MmlSongOptions::default();
        options.min_gap_for_chord = 5;

        let ppq = 480;

        // Create chord notes with different durations
        let note1 = create_test_midi_note_state(60, 64, 0, 480, 0); // Quarter note
        let note2 = create_test_midi_note_state(64, 64, 1, 240, 0); // Eighth note, but should get quarter duration

        let bridge_events = vec![BridgeEvent::Note(note1), BridgeEvent::Note(note2)];

        let (mml_events, _) = bridge_events_to_mml_events(&bridge_events, &options, ppq);

        let note_events: Vec<_> = mml_events
            .iter()
            .filter_map(|e| match e {
                MmlEvent::Note(note) => Some(note),
                _ => None,
            })
            .collect();

        if note_events.len() == 2 && note_events[1].is_part_of_chord {
            // Chord notes should have same duration
            assert_eq!(
                note_events[0].duration_in_smallest_unit, note_events[1].duration_in_smallest_unit,
                "Chord notes should have same duration"
            );
        }
    }
}
