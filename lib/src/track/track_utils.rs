use crate::{Instrument, Note, Track, utils};
use midly::{TrackEventKind, MetaMessage, MidiMessage};
use std::convert::TryInto;

pub fn split_track(track: &Track) -> (Track, Track) {
    let (mut track_a, mut track_b) = split_track_by_override(track);

    if track_a.mml_note_length > 3000 && track_b.mml_note_length > 3000 {
        equalize_tracks(&mut track_a, &mut track_b);
    }

    (track_a, track_b)
}

/// Equalize the number of notes in the two tracks so that they are equal
pub fn equalize_tracks(track_a: &mut Track, track_b: &mut Track) {
    let equalize = |a: &mut Track, b: &mut Track, gap: usize| {
        let mut mml_counter = 0usize;
        let mut index_counter = 0usize;

        for (index, note) in a.notes.iter().enumerate() {
            mml_counter += note.count_mml_note();

            if mml_counter >= gap {
                index_counter = index + 1;
                break;
            }
        }
        
        let (left, right) = a.notes.split_at(index_counter);
        let mut left = left.to_vec();

        a.notes = right.to_vec();
        a.update_events();
        a.update_mml_note_length();

        b.notes.append(&mut left);
        b.notes.sort();
        b.update_events();
        b.update_mml_note_length();
    };

    let gap = track_a.mml_note_length as isize - track_b.mml_note_length as isize;
    if gap > 0 {
        equalize(track_a, track_b, gap as usize);
    } else {
        equalize(track_b, track_a, gap.abs() as usize);
    }
}

pub fn split_track_by_override(track: &Track) -> (Track, Track) {
    let mut max_end_position = 0u32;
    let mut notes_a: Vec<Note> = Vec::new();
    let mut notes_b: Vec<Note> = Vec::new();

    for i in 0..track.notes.len() {
        let current_note = track.notes.get(i).unwrap();
        let current_end_position =
            current_note.position_in_smallest_unit + current_note.duration_in_smallest_unit;

        if current_end_position > max_end_position {
            max_end_position = current_end_position;
        }

        if i > 0 {
            let before_note = track.notes.get(i - 1).unwrap();

            if utils::is_can_connect_to_chord(current_note, before_note) {
                notes_a.push(current_note.to_owned());
            } else {
                if current_note.position_in_smallest_unit < max_end_position {
                    notes_b.push(current_note.to_owned());
                } else {
                    notes_a.push(current_note.to_owned());
                }
            }
        } else {
            notes_a.push(current_note.to_owned());
        }
    }

    (
        Track::from_notes(
            format!("{}.0", &track.name),
            track.ppq,
            track.bpm,
            track.instrument.to_owned(),
            notes_a,
        ),
        Track::from_notes(
            format!("{}.1", &track.name),
            track.ppq,
            track.bpm,
            track.instrument.to_owned(),
            notes_b,
        ),
    )
}

pub fn get_bpm_from_smf_track(smf_track: &midly::Track) -> Option<u16> {
    for smf_event in smf_track.iter() {
        match smf_event.kind {
            TrackEventKind::Meta(message) => match message {
                MetaMessage::Tempo(tempo) => {
                    let bpm = (60_000_000 / tempo.as_int()).try_into().unwrap();
                    return Some(bpm);
                }
                _ => (),
            },
            _ => (),
        }
    }

    None
}

pub fn get_instrument_from_track(smf_track: &midly::Track) -> Instrument {
    let mut midi_channel: u8 = 0;
    let mut instrument_id: u8 = 0;

    for smf_event in smf_track.iter() {
        match smf_event.kind {
            TrackEventKind::Midi { message, channel } => match message {
                MidiMessage::ProgramChange { program } => {
                    instrument_id = program.as_int();
                    midi_channel = channel.as_int() + 1;
                }
                _ => (),
            },
            _ => (),
        }
    }
    
    Instrument::new(instrument_id, midi_channel)
}
