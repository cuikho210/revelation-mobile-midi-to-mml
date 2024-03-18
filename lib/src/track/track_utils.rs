use crate::{Instrument, Note, Track, utils};
use midly::{TrackEventKind, MetaMessage, MidiMessage};
use std::convert::TryInto;

pub fn split_notes_by_override(track: &Track) -> (Track, Track) {
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
