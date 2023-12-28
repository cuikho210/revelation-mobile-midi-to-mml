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
    pub notes: Vec<Note>,
    pub ppq: u16,
    pub bpm: u16,
}

impl Track {
    pub fn new(smf_track: &midly::Track, ppq: u16, bpm: &mut u16) -> Self {
        if let Some(new_bpm) = get_bpm_from_smf_track(smf_track) {
            *bpm = new_bpm;
        };

        let notes = get_notes_from_smf_track(smf_track, ppq);

        let mut result = Self {
            events: Vec::new(),
            notes,
            ppq,
            bpm: *bpm
        };

        result.update_events();
        result
    }

    pub fn to_mml(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        result.push(TrackEvent::SetTempo(self.bpm).to_mml());

        for event in self.events.iter() {
            result.push(event.to_mml());
        }

        result.join("")
    }

    /// Join other track to this track
    pub fn concat(&mut self, other: &Self) {
        self.notes.append(&mut other.notes.to_owned());

        // Sort by position_in_smallest_unit
        self.notes.sort();
        self.update_events();
    }

    fn update_events(&mut self) {
        self.events = get_events_from_notes(&mut self.notes);
        fix_chord_duration(&mut self.events);
        fix_note_position(&mut self.events);
    }
}

fn fix_note_position(events: &mut Vec<TrackEvent>) {
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
                            note.duration_in_smallest_unit = (note_duration - redundant).try_into().unwrap();
                            redundant = 0;
                        } else {
                            note.duration_in_smallest_unit = 1;
                            redundant -= note_duration - 1;
                        }
                    }

                    latest_duration = note.duration_in_smallest_unit;
                }
            }
            _ => ()
        }
    }
}

fn fix_chord_duration(events: &mut Vec<TrackEvent>) {
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
                    if position_diff >= max_end_position {
                        position_diff = position_diff - max_end_position;
                    } else {
                        position_diff = 0;
                    }
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

                // Velocity
                if note.velocity != before_note.velocity {
                    events.push(TrackEvent::SetVelocity(note.velocity));
                }
            }
        } else {
            octave_event = Some(TrackEvent::SetOctave(note.octave));
            events.push(TrackEvent::SetVelocity(note.velocity));
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
                        let velocity = vel.as_int() / 16 + 6;

                        if vel.as_int() > 0 {
                            create_note(
                                midi_key,
                                velocity,
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
    velocity: u8,
    current_ticks: u32,
    notes: &mut Vec<Note>,
    holding_notes: &mut HashMap<u8, usize>,
    ppq: u16,
) {
    let note = Note::new(
        ppq,
        midi_key,
        velocity,
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
