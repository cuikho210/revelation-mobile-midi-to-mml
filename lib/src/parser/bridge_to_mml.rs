use crate::{
    Instrument,
    mml_event::{BridgeEvent, MmlEvent},
    mml_note::MmlNote,
    mml_song::MmlSongOptions,
    utils::{compute_position_in_smallest_unit, tick_to_smallest_unit},
};

pub fn bridge_events_to_mml_events(
    bridge_events: &[BridgeEvent],
    options: &MmlSongOptions,
    ppq: u16,
) -> (Vec<MmlEvent>, Option<Instrument>) {
    let (mut mml_events, instrument) = bridge_events_to_raw_mml_events(bridge_events, options, ppq);

    normalize_events(&mut mml_events);
    fix_events_position(&mut mml_events);
    normalize_events(&mut mml_events);
    update_chord_duration(&mut mml_events);
    update_note_mml(&mut mml_events, options.smallest_unit);

    (mml_events, instrument)
}

fn bridge_events_to_raw_mml_events(
    bridge_events: &[BridgeEvent],
    options: &MmlSongOptions,
    ppq: u16,
) -> (Vec<MmlEvent>, Option<Instrument>) {
    let mut mml_events: Vec<MmlEvent> = Vec::new();

    // The note immediately preceding the current note.
    // Can be part of a chord or not.
    let mut before_note: Option<MmlNote> = None;

    // The first onset note of the most recent chord.
    let mut first_onset_note_index: Option<usize> = None;

    let mut instrument = None;

    for event in bridge_events.iter() {
        match event {
            BridgeEvent::Tempo(tempo, state) => {
                let pos = tick_to_smallest_unit(state.position_in_tick, ppq, options.smallest_unit);
                mml_events.push(MmlEvent::Tempo(tempo.to_owned(), pos));
            }
            BridgeEvent::ProgramChange(dest_instrument, _) => {
                instrument = Some(dest_instrument.to_owned());
            }
            BridgeEvent::Note(midi_state) => {
                let mut note = MmlNote::from_midi_state(midi_state.to_owned(), options, ppq, false);

                if let Some(before_note) = &before_note {
                    if let Some(index) = first_onset_note_index {
                        handle_bridge_note_events(
                            &mut mml_events,
                            &mut note,
                            index,
                            options.min_gap_for_chord,
                        );
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

                if !note.is_part_of_chord {
                    first_onset_note_index = Some(mml_events.len());
                }

                mml_events.push(MmlEvent::Note(note));
            }
        }
    }

    (mml_events, instrument)
}

fn handle_bridge_note_events(
    mml_events: &mut Vec<MmlEvent>,
    note: &mut MmlNote,
    first_onset_note_index: usize,
    min_gap_for_chord: u8,
) {
    let Some(MmlEvent::Note(b_note)) = mml_events.get_mut(first_onset_note_index) else {
        return;
    };

    let b_note_end = b_note.position_in_smallest_unit + b_note.duration_in_smallest_unit;
    let gap = note.position_in_smallest_unit as isize - b_note_end as isize;

    if gap > 0 {
        // We need to fill the gap with a rest:
        // B---------B
        //                  C--------C
        //            R----R

        mml_events.push(MmlEvent::Rest(gap as usize));
    } else if gap < 0 {
        // Gap less than 0 means notes like this:
        // B---------------B
        //            C---------------C
        //
        // or like this 😱😱:
        // B---------------B
        //     C--------C

        let note_pos_isize = note.position_in_smallest_unit as isize;
        let before_note_pos_isize = b_note.position_in_smallest_unit as isize;
        let start_pos_gap = note_pos_isize - before_note_pos_isize;

        if start_pos_gap <= min_gap_for_chord as isize {
            note.is_part_of_chord = true;
            mml_events.push(MmlEvent::ConnectChord);
        } else {
            // If the `before_note` is part of a chord:
            // B1---------B1
            // B2---------B2
            //      C------------C
            //
            // Then, after updating:
            // B1---------B1
            // B2-B2
            //      C------------C
            //
            // -> This does not make sense.
            //
            // So, we need to update the first onset note instead:
            // B1-B1
            // B2---------B2
            //      C------------C
            //
            // Then after update_chord_duration:
            // B1-B1
            // B2-B2
            //      C------------C

            let overrided = (before_note_pos_isize + b_note.duration_in_smallest_unit as isize)
                - note_pos_isize;
            if overrided > 0 {
                b_note.duration_in_smallest_unit =
                    b_note.duration_in_smallest_unit - overrided as usize;
            }
        }
    }
}

fn update_note_mml(events: &mut [MmlEvent], smallest_unit: usize) {
    for event in events.iter_mut() {
        if let MmlEvent::Note(note) = event {
            note.update_mml_string(smallest_unit);
        }
    }
}

pub fn fix_events_position(events: &mut Vec<MmlEvent>) {
    let mut i = 0;
    while i < events.len() {
        let current_event = events.get(i).unwrap();

        match &current_event {
            MmlEvent::Note(note) => {
                if !note.is_part_of_chord {
                    let new_i = fix_event_position(events, i);
                    if new_i != i {
                        i = new_i;
                    }
                }
            }
            MmlEvent::Tempo(_, _) => {
                let new_i = fix_event_position(events, i);
                if new_i != i {
                    i = new_i;
                }
            }
            _ => (),
        };

        i += 1;
    }
}

fn fix_event_position(events: &mut Vec<MmlEvent>, event_index: usize) -> usize {
    let Some(event) = events.get(event_index) else {
        return event_index;
    };
    let Some(expect_pos) = event.get_position() else {
        return event_index;
    };
    let mut current_pos = compute_position_in_smallest_unit(events, event_index);

    if expect_pos == current_pos {
        return event_index;
    }

    let mut i = event_index;
    while i > 0 {
        i -= 1;

        let e = events.get_mut(i).unwrap();
        if e.is_part_of_chord() {
            continue;
        }

        if expect_pos > current_pos {
            let to_incre = expect_pos - current_pos;
            events.insert(i + 1, MmlEvent::Rest(to_incre));
            return event_index + 1;
        } else {
            let to_decre = current_pos - expect_pos;
            let Some(e_dur) = e.get_duration() else {
                continue;
            };

            if e_dur > to_decre {
                e.set_duration(e_dur - to_decre);
            } else if let MmlEvent::Note(note) = e {
                note.is_part_of_chord = true;
            } else if let MmlEvent::Rest(_) = e {
                events.remove(i);
                return event_index - 1;
            }

            current_pos = compute_position_in_smallest_unit(events, event_index);
            if expect_pos == current_pos {
                return event_index;
            }
        }
    }

    if expect_pos > current_pos {
        events.insert(0, MmlEvent::Rest(expect_pos - current_pos));
        return event_index + 1;
    }

    event_index
}

fn normalize_events(events: &mut Vec<MmlEvent>) {
    let mut i = 0;
    let mut before_note_i: Option<usize> = None;

    while i < events.len() {
        let current_e = events.get(i).unwrap().clone();

        match current_e {
            MmlEvent::Note(note) => {
                if note.is_part_of_chord {
                    if !has_a_connect_chord_event(events, i) {
                        if let Some(before_note_i) = before_note_i {
                            events.insert(before_note_i + 1, MmlEvent::ConnectChord);
                            i += 1;
                        } else {
                            // TODO: Handle case where chord note has no preceding note
                            println!("Note {note:?} at {i}");
                            panic!();
                        }
                    }
                } else if note.duration_in_smallest_unit == 0 {
                    events.remove(i);
                    i -= 1;
                } else {
                    before_note_i = Some(i);
                }
            }
            MmlEvent::Rest(rest) => {
                if rest == 0 {
                    events.remove(i);
                    i -= 1;
                }
            }
            MmlEvent::Tempo(_, _) => {
                if i > 0
                    && let Some(MmlEvent::Tempo(_, _)) = events.get(i - 1)
                {
                    events.remove(i - 1);
                    i -= 1;
                }
            }
            _ => {}
        }

        i += 1;
    }
}

pub fn has_a_connect_chord_event(events: &[MmlEvent], index: usize) -> bool {
    for i in (0..index).rev() {
        let e = events.get(i).unwrap();
        match e {
            MmlEvent::ConnectChord => return true,
            MmlEvent::Note(_) => return false,
            _ => (),
        }
    }
    false
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
    use crate::{
        MmlEvent, MmlNote,
        parser::bridge_to_mml::{
            bridge_events_to_raw_mml_events, fix_event_position, fix_events_position,
            has_a_connect_chord_event, normalize_events, update_chord_duration,
        },
        test_utils::{self, MIDI_PATHS},
        utils::compute_position_in_smallest_unit,
    };

    #[test]
    fn test_update_chord_duration() {
        for path in MIDI_PATHS {
            println!("Testing {path}");

            let (bridge_events, options, ppq) = test_utils::setup_bridge_events(path);
            let (mut events, _) = bridge_events_to_raw_mml_events(&bridge_events, &options, ppq);

            normalize_events(&mut events);
            fix_events_position(&mut events);
            normalize_events(&mut events);
            update_chord_duration(&mut events);

            let mut before_note_duration: Option<usize> = None;
            for e in events.iter() {
                if let MmlEvent::Note(note) = e {
                    if note.is_part_of_chord {
                        assert_eq!(
                            note.duration_in_smallest_unit,
                            before_note_duration.unwrap(),
                            "Chord note duration mismatch"
                        );
                    } else {
                        before_note_duration = Some(note.duration_in_smallest_unit);
                    }
                }
            }
        }
    }

    #[test]
    fn test_fix_events_position() {
        for path in MIDI_PATHS {
            println!("Testing {path}");

            let (bridge_events, options, ppq) = test_utils::setup_bridge_events(path);
            let (mut events, _) = bridge_events_to_raw_mml_events(&bridge_events, &options, ppq);
            normalize_events(&mut events);
            fix_events_position(&mut events);
            normalize_events(&mut events);

            for (i, e) in events.iter().enumerate() {
                if !e.is_part_of_chord()
                    && let Some(expected) = e.get_position()
                {
                    let computed = compute_position_in_smallest_unit(&events, i);

                    if computed != expected {
                        println!("MML Events -------------------");
                        for (i, e) in events[..=i].iter().enumerate() {
                            println!("{i}: {e:?}");
                        }
                        println!("-------------------");
                        println!("{i}: Expected {expected} but computed {computed}");
                        panic!("computed != expected");
                    }
                }
            }
        }
    }

    #[test]
    fn test_fix_event_position() {
        let (bridge_events, options, ppq) = test_utils::setup_bridge_events(MIDI_PATHS[1]);
        let (mut events, _) = bridge_events_to_raw_mml_events(&bridge_events, &options, ppq);
        normalize_events(&mut events);

        let i = fix_event_position(&mut events, 112);
        let e = events.get(i).unwrap();
        println!("Event: {e:?} at {i}");
        let computed = compute_position_in_smallest_unit(&events, i);
        let expected = e.get_position().unwrap();
        assert_eq!(computed, expected);
    }

    #[test]
    fn test_normalize_events() {
        for path in MIDI_PATHS {
            println!("Testing {path}");

            let (bridge_events, options, ppq) = test_utils::setup_bridge_events(path);
            let (mut events, _) = bridge_events_to_raw_mml_events(&bridge_events, &options, ppq);

            let duration_before = compute_position_in_smallest_unit(&events, events.len());

            normalize_events(&mut events);

            let duration_after = compute_position_in_smallest_unit(&events, events.len());
            assert_eq!(duration_after, duration_before);

            for (i, e) in events.iter().enumerate() {
                match e {
                    MmlEvent::Note(note) => {
                        if note.is_part_of_chord {
                            assert!(has_a_connect_chord_event(&events, i));
                            continue;
                        }
                        assert!(note.duration_in_smallest_unit > 0);
                    }
                    MmlEvent::Rest(rest) => assert!(*rest > 0),
                    MmlEvent::Tempo(_, _) => {
                        if i > 0
                            && let Some(MmlEvent::Tempo(_, _)) = events.get(i - 1)
                        {
                            panic!("Duplicated tempo");
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    #[test]
    fn test_bridge_events_to_raw_mml_events() {
        for path in MIDI_PATHS {
            println!("Testing {path}");
            let (bridge_events, options, ppq) = test_utils::setup_bridge_events(path);
            let (events, _) = bridge_events_to_raw_mml_events(&bridge_events, &options, ppq);

            let mut before_note: Option<MmlNote> = None;
            for (i, e) in events.iter().enumerate() {
                match e {
                    MmlEvent::Note(note) => {
                        if note.is_part_of_chord {
                            assert!(has_a_connect_chord_event(&events, i));
                            continue;
                        }

                        if let Some(b_note) = &before_note {
                            println!("Asserting {i}:");
                            println!("before note: {b_note:?}");
                            println!("note: {note:?}");
                            assert!(
                                b_note.position_in_smallest_unit + b_note.duration_in_smallest_unit
                                    <= note.position_in_smallest_unit
                            );
                        }

                        before_note = Some(note.to_owned());
                    }
                    _ => (),
                }
            }
        }
    }
}
