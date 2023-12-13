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

        let mut notes = get_notes_from_smf_track(smf_track);
        let events = get_events_from_notes(&mut notes, ppq);

        Self {
            events,
            ppq,
            bpm: *bpm
        }
    }

    pub fn to_mml(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        result.push(TrackEvent::SetTempo(self.bpm).to_mml(&self.ppq));

        for event in self.events.iter() {
            result.push(event.to_mml(&self.ppq));
        }

        result.join("")
    }
}

fn get_events_from_notes(notes: &mut Vec<Note>, ppq: u16) -> Vec<TrackEvent> {
    let mut events: Vec<TrackEvent> = Vec::new();

    for index in 0..notes.len() {
        let note = notes.get(index).unwrap().to_owned();
        let mut position_diff = note.position_in_tick;
        let mut octave_event: Option<TrackEvent> = None;

        if index > 0 {
            if let Some(before_note) = notes.get_mut(index - 1) {
                let before_note_end_position = before_note.position_in_tick + before_note.duration_in_tick;

                // If while another note is playing
                if note.position_in_tick < before_note_end_position {
                    position_diff = 0;

                    utils::connect_to_chord_or_cut_before_note(
                        &mut events,
                        ppq, 
                        &note,
                        before_note
                    );
                } else {
                    position_diff = position_diff - before_note_end_position;
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

        events.push(TrackEvent::SetNote(note.to_owned()));
    }

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

fn get_notes_from_smf_track(smf_track: &midly::Track) -> Vec<Note> {
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
                            );
                        } else {
                            update_note(
                                midi_key,
                                current_ticks,
                                &mut result,
                                &mut holding_notes,
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
) {
    let note = Note::new(midi_key, current_ticks);
    holding_notes.insert(midi_key, notes.len());
    notes.push(note);
}

fn update_note(
    midi_key: u8,
    current_ticks: u32,
    notes: &mut Vec<Note>,
    holding_notes: &mut HashMap<u8, usize>,
) {
    if let Some(index) = holding_notes.get(&midi_key) {
        if let Some(note) = notes.get_mut(index.to_owned()) {
            note.duration_in_tick = current_ticks - note.position_in_tick;
        }
    }
}
