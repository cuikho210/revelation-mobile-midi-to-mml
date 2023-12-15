use std::collections::HashMap;
use midly::{TrackEventKind, MetaMessage, MidiMessage};
use crate::{
    track_event::TrackEvent,
    note::Note, utils,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub events: Vec<TrackEvent>,
    pub ppq: u16,
    pub bpm: u16,
}

impl Track {
    pub fn new(smf_track: &midly::Track, ppq: u16, bpm: &mut u16) -> Self {
        if let Some(new_bpm) = get_bpm_from_smf_track(smf_track) {
            *bpm = new_bpm;
        };

        let mut notes = get_notes_from_smf_track(smf_track, ppq);
        let events = get_events_from_notes(&mut notes);

        Self {
            events,
            ppq,
            bpm: *bpm
        }
    }

    pub fn to_mml(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        result.push(TrackEvent::SetTempo(self.bpm).to_mml());

        for event in self.events.iter() {
            result.push(event.to_mml());
        }

        result.join("")
    }
}

fn get_events_from_notes(notes: &Vec<Note>) -> Vec<TrackEvent> {
    let mut events: Vec<TrackEvent> = Vec::new();
    let mut max_end_position = 0u32;

    for index in 0..notes.len() {
        let note = notes.get(index).unwrap();
        let mut position_diff = note.position_in_smallest_unit;
        let mut octave_event: Option<TrackEvent> = None;

        if index > 0 {
            if let Some(before_note) = notes.get(index - 1) {
                let before_note_end_position = before_note.position_in_smallest_unit + before_note.duration_in_smallest_unit;
                let mut is_connected_to_chord = false;

                // If while another note is playing
                if note.position_in_smallest_unit < before_note_end_position {
                    position_diff = 0;

                    is_connected_to_chord = utils::try_connect_to_chord(
                        &mut events,
                        note,
                        before_note,
                    );
                } else {
                    position_diff = position_diff - before_note_end_position;
                }

                // Cut previous notes
                if !is_connected_to_chord && note.position_in_smallest_unit < max_end_position {
                    utils::cut_previous_notes(
                        &mut events,
                        note.position_in_smallest_unit,
                    );

                    max_end_position = note.position_in_smallest_unit + note.duration_in_smallest_unit;
                }

                // Octave event
                let octave_diff = note.octave as i8 - before_note.octave as i8;

                if octave_diff == 1 {
                    octave_event = Some(TrackEvent::IncreOctave);
                } else if octave_diff == -1 {
                    octave_event = Some(TrackEvent::DecreOctave);
                } else if octave_diff != 0 {
                    octave_event = Some(TrackEvent::SetOctave(note.octave));
                }
            }
        } else {
            octave_event = Some(TrackEvent::SetOctave(note.octave));
        }

        if position_diff > 0 {
            events.push(TrackEvent::SetRest(position_diff));
        }

        if let Some(octave_event) = octave_event {
            events.push(octave_event);
        }

        let current_end_position = note.position_in_smallest_unit + note.duration_in_smallest_unit;
        if current_end_position > max_end_position {
            max_end_position = current_end_position;
        }

        events.push(TrackEvent::SetNote(note.to_owned()));
    }

    println!("{:#?}", &events);
    events
}

fn get_bpm_from_smf_track(smf_track: &midly::Track) -> Option<u16> {
    for smf_event in smf_track.iter() {
        match smf_event.kind {
            TrackEventKind::Meta(message) => {
                match message {
                    MetaMessage::Tempo(tempo) => {
                        let bpm = (60_000_000 / tempo.as_int()).try_into().unwrap();
                        return Some(bpm);
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }

    None
}

fn get_notes_from_smf_track(smf_track: &midly::Track, ppq: u16) -> Vec<Note> {
    let mut result: Vec<Note> = Vec::new();
    let mut holding_notes: HashMap<u8, usize> = HashMap::new();
    let mut current_ticks = 0u32;

    for midi_event in smf_track.iter() {
        let delta = midi_event.delta.as_int();
        current_ticks += delta;

        match midi_event.kind {
            TrackEventKind::Midi { message, .. } => {
                match message {
                    MidiMessage::NoteOn { key, vel } => {
                        let midi_key = key.as_int();

                        if vel.as_int() > 0 {
                            create_note(
                                midi_key,
                                current_ticks,
                                &mut result,
                                &mut holding_notes,
                                ppq,
                            );
                        } else {
                            update_note(
                                midi_key,
                                current_ticks,
                                &mut result,
                                &mut holding_notes,
                                ppq,
                            );
                        }
                    }
                    MidiMessage::NoteOff { key, .. } => {
                        let midi_key = key.as_int();

                        update_note(
                            midi_key,
                            current_ticks,
                            &mut result,
                            &mut holding_notes,
                            ppq,
                        );
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }

    result
}

fn create_note(
    midi_key: u8,
    current_ticks: u32,
    notes: &mut Vec<Note>,
    holding_notes: &mut HashMap<u8, usize>,
    ppq: u16,
) {
    let note = Note::new(
        ppq,
        midi_key,
        current_ticks,
    );

    holding_notes.insert(midi_key, notes.len());
    notes.push(note);
}

fn update_note(
    midi_key: u8,
    current_ticks: u32,
    notes: &mut Vec<Note>,
    holding_notes: &mut HashMap<u8, usize>,
    ppq: u16,
) {
    if let Some(index) = holding_notes.get(&midi_key) {
        if let Some(note) = notes.get_mut(index.to_owned()) {
            let duration = current_ticks - note.position_in_tick;
            let duration_in_smallest_unit = utils::tick_to_smallest_unit(duration, ppq);
            note.duration_in_tick = duration;
            note.duration_in_smallest_unit = duration_in_smallest_unit;
        }
    }
}
