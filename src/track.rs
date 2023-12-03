use std::collections::HashMap;
use midly::{TrackEventKind, MetaMessage, MidiMessage};
use crate::{
    track_event::TrackEvent,
    note::Note,
};

#[derive(Debug, Clone)]
pub struct Track {
    events: Vec<TrackEvent>,
    ppq: u16,
    bpm: u16,
}

impl Track {
    pub fn new(smf_track: &midly::Track, ppq: u16, bpm: &mut u16) -> Self {
        let mut events: Vec<TrackEvent> = Vec::new();
        let mut holding_notes: HashMap<u8, usize> = HashMap::new();
        let mut current_tick: u32 = 0;

        for smf_event in smf_track.iter() {
            let delta = smf_event.delta.as_int();
            current_tick += delta;

            match smf_event.kind {
                TrackEventKind::Meta(message) => {
                    match_meta_event(&message, bpm);
                }
                TrackEventKind::Midi { message , .. } => {
                    match_midi_event(
                        &message,
                        &mut events,
                        &mut holding_notes,
                        &current_tick,
                    );
                }
                _ => ()
            }
        }

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

fn match_meta_event(
    message: &MetaMessage,
    bpm: &mut u16,
) {
    match message {
        MetaMessage::Tempo(tempo) => {
            *bpm = (60_000_000 / tempo.as_int()).try_into().unwrap();
        }
        _ => ()
    }
}

fn match_midi_event(
    message: &MidiMessage,
    events: &mut Vec<TrackEvent>,
    holding_notes: &mut HashMap<u8, usize>,
    current_ticks: &u32,
) {
    match message {
        MidiMessage::NoteOn { key, vel } => {
            let midi_key = key.as_int();

            if vel.as_int() == 0 {
                update_note(
                    midi_key,
                    events,
                    holding_notes,
                    current_ticks,
                );
            } else {
                create_note(
                    midi_key,
                    events,
                    holding_notes,
                    current_ticks,
                );
            }
        }
        MidiMessage::NoteOff { key, .. } => {
            let midi_key = key.as_int();

            update_note(
                midi_key,
                events,
                holding_notes,
                current_ticks,
            );
        }
        _ => ()
    }
}

fn create_note(
    midi_key: u8,
    events: &mut Vec<TrackEvent>,
    holding_notes: &mut HashMap<u8, usize>,
    current_ticks: &u32,
) {
    let note  = Note::new(
        midi_key,
        current_ticks.to_owned(),
    );

    let mut position_diff: i32 = note.position_in_tick.try_into().unwrap();

    if let Some(before_note) = get_before_note(events) {
        let before_note_end_position: i32 = (before_note.position_in_tick + before_note.duration_in_tick).try_into().unwrap();
        position_diff = position_diff - before_note_end_position;

        // Chord
        if holding_notes.len() > 0 || position_diff <= 0 {
            events.push(TrackEvent::ConnectChord);
        }
        // Rest
        else {
            events.push(TrackEvent::SetRest(position_diff.try_into().unwrap()));
        }

        // Octave
        let octave_diff = note.octave as i8 - before_note.octave as i8;

        if octave_diff == 1 {
            events.push(TrackEvent::IncreOctave);
        } else if octave_diff == -1 {
            events.push(TrackEvent::DecreOctave);
        } else {
            events.push(TrackEvent::SetOctave(note.octave));
        }
    } else {
        // Rest
        if position_diff > 0 {
            events.push(TrackEvent::SetRest(position_diff.try_into().unwrap()));
        }

        events.push(TrackEvent::SetOctave(note.octave));
    }

    // Set note
    holding_notes.insert(midi_key, events.len());
    events.push(TrackEvent::SetNote(note));
}

fn update_note(
    midi_key: u8,
    events: &mut Vec<TrackEvent>,
    holding_notes: &mut HashMap<u8, usize>,
    current_ticks: &u32,
) {
    let index = holding_notes.get(&midi_key);
    if let Some(index) = index {
        if let Some(event) = events.get_mut(index.to_owned()) {
            if let TrackEvent::SetNote(note) = event {
                let duration_in_ticks = current_ticks - note.position_in_tick;
                note.duration_in_tick = duration_in_ticks;
            }
        }
    }

    holding_notes.remove(&midi_key);
}

fn get_before_note(events: &Vec<TrackEvent>) -> Option<Note> {
    if events.len() == 0 {
        return None;
    }

    for event in events.iter().rev() {
        match event {
            TrackEvent::SetNote(note) => return Some(note.to_owned()),
            _ => ()
        }
    }

    None
}