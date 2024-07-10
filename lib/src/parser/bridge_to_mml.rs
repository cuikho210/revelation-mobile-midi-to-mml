use crate::{mml_event::{BridgeEvent, MmlEvent}, mml_note::MmlNote, mml_song::MmlSongOptions, Instrument};

pub fn bridge_events_to_mml_events(
    bridge_events: Vec<BridgeEvent>,
    options: &MmlSongOptions,
    ppq: u16,
) -> (Vec<MmlEvent>, Instrument) {
    let mut mml_events: Vec<MmlEvent> = Vec::new();
    let mut before_note: Option<MmlNote> = None;
    let mut instrument = Instrument::default();

    for event in bridge_events {
        match event {
            BridgeEvent::Tempo(tempo, .. ) => {
                mml_events.push(MmlEvent::Tempo(tempo));
            }
            BridgeEvent::Note(midi_state) => {
                let mut note = MmlNote::from_midi_state(midi_state, options, ppq, false);
                
                if let Some(before_note) = &before_note {
                    // Rest and chord
                    let before_note_end = before_note.position_in_smallest_unit + before_note.duration_in_smallest_unit;
                    let position_diff = note.position_in_smallest_unit as isize - before_note_end as isize;

                    if position_diff > 0 {
                        mml_events.push(MmlEvent::Rest(position_diff as u8));
                    } else if position_diff < 0 {
                        let note_pos_isize = note.position_in_smallest_unit as isize;
                        let before_note_pos_isize = before_note.position_in_smallest_unit as isize;
                        let start_pos_diff = (note_pos_isize - before_note_pos_isize).abs();

                        if start_pos_diff <= 0 {
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
                    mml_events.push(MmlEvent::Velocity(note.velocity));
                    mml_events.push(MmlEvent::Octave(note.octave));
                }

                before_note = Some(note.to_owned());
                mml_events.push(MmlEvent::Note(note));
            }
            BridgeEvent::ProgramChange(dest_instrument, _) => {
                instrument = dest_instrument;
            }
        }
    }

    fix_note_position(&mut mml_events);
    update_chord_duration(&mut mml_events);

    (mml_events, instrument)
}

fn fix_note_position(events: &mut Vec<MmlEvent>) {
    let mut current_position = 0usize;
    
    for i in 0..events.len() {
        let current_event = events.get(i).unwrap().to_owned();

        match current_event {
            MmlEvent::Rest(rest) => {
                current_position += rest as usize;
            }
            MmlEvent::Note(note) => {
                if note.is_part_of_chord {
                    continue;
                }

                let position_diff = note.position_in_smallest_unit as isize - current_position as isize;
                if position_diff != 0 {
                    modify_before_duration(events, i, position_diff);
                    current_position = note.position_in_smallest_unit;
                }

                current_position += note.duration_in_smallest_unit;
            }
            _ => ()
        }
    }
}

fn modify_before_duration(events: &mut Vec<MmlEvent>, mut current_index: usize, mut to_modify: isize) {
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
                        *rest = new_rest as u8;
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
                        note.duration_in_smallest_unit = 1;
                        to_modify += duration_isize - 1;
                    }
                }
                _ => ()
            }
        }
    }
}

pub fn update_chord_duration(events: &mut Vec<MmlEvent>) {
    let mut before_note: Option<MmlNote> = None;

    for i in 0..events.len() {
        let event = events.get_mut(i).unwrap();

        match event {
            MmlEvent::Note(note) => {
                if note.is_part_of_chord {
                    if let Some(before_note) = &before_note {
                        note.duration_in_smallest_unit = before_note.duration_in_smallest_unit;
                    }
                    continue;
                }

                before_note = Some(note.to_owned());
            }
            _ => ()
        }
    }
}
