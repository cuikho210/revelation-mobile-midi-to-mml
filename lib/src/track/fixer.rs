use std::convert::TryInto;
use crate::{
    note::Note,
    track::TrackEvent,
};

pub fn modify_note_velocity(notes: &mut Vec<Note>, diff: u8) {
    if diff > 0 {
        for note in notes.iter_mut() {
            note.velocity += diff;
        }
    }
}

pub fn fix_note_position(events: &mut Vec<TrackEvent>) {
    let mut current_position = 0u32;
    let mut connect_to_chord = false;
    let mut latest_duration = 0u32;
    let mut redundant = 0i32;

    for event in events.iter_mut() {
        match event {
            TrackEvent::ConnectChord => {
                connect_to_chord = true;
            }
            TrackEvent::SetRest(rest) => {
                current_position += latest_duration;
                let rest_i32: i32 = rest.to_owned().try_into().unwrap();

                if redundant > 0 && redundant >= rest_i32 {
                    *rest = 0;
                    redundant -= rest_i32;
                } else {
                    redundant = 0;
                    *rest = (rest_i32 - redundant).try_into().unwrap();
                }

                latest_duration = rest.to_owned();
            }
            TrackEvent::SetNote(note) => {
                if connect_to_chord {
                    note.duration_in_smallest_unit = latest_duration;
                    connect_to_chord = false;
                } else {
                    current_position += latest_duration;

                    let note_duration: i32 = note.duration_in_smallest_unit.try_into().unwrap();
                    let note_position: i32 = note.position_in_smallest_unit.try_into().unwrap();
                    let current_position_i32: i32 = current_position.try_into().unwrap();
                    redundant += current_position_i32 - note_position;

                    if redundant != 0 {
                        if redundant < note_duration {
                            note.duration_in_smallest_unit =
                                (note_duration - redundant).try_into().unwrap();
                            redundant = 0;
                        } else {
                            note.duration_in_smallest_unit = 1;
                            redundant -= note_duration - 1;
                        }
                    }

                    latest_duration = note.duration_in_smallest_unit;
                }
            }
            _ => (),
        }
    }
}

pub fn fix_chord_duration(events: &mut Vec<TrackEvent>) {
    let mut current_chord: Vec<usize> = Vec::new();

    for i in 0..events.len() {
        let event = events.get(i).unwrap();

        if let TrackEvent::ConnectChord = event {
            current_chord.push(i - 1);
        } else if !current_chord.is_empty() {
            if let TrackEvent::SetNote(_) = event {
                let mut is_chord_end = false;

                if let Some(event_after) = events.get(i + 1) {
                    match event_after {
                        TrackEvent::ConnectChord => (),
                        _ => is_chord_end = true,
                    }
                } else {
                    is_chord_end = true;
                }

                if is_chord_end {
                    current_chord.push(i);
                    let mut max_duration = 0;

                    for i in current_chord.iter() {
                        if let TrackEvent::SetNote(note) = events.get(i.to_owned()).unwrap() {
                            if note.duration_in_smallest_unit > max_duration {
                                max_duration = note.duration_in_smallest_unit;
                            }
                        }
                    }

                    for i in current_chord.iter() {
                        if let TrackEvent::SetNote(note) = events.get_mut(i.to_owned()).unwrap() {
                            note.duration_in_smallest_unit = max_duration;
                        }
                    }

                    current_chord.clear();
                }
            }
        }
    }
}
