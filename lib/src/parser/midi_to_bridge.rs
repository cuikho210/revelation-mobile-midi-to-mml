use std::collections::HashMap;
use midly::{MetaMessage, MidiMessage, Track as MidiTrack, TrackEventKind};
use crate::{mml_event::{BridgeEvent, MidiNoteState, MidiState}, Instrument};

pub fn bridge_meta_from_midi_track(midi_track: &MidiTrack) -> Vec<BridgeEvent> {
    let mut meta_events: Vec<BridgeEvent> = Vec::new();
    let mut current_ticks = 0usize;

    for midi_event in midi_track.iter() {
        let delta = midi_event.delta.as_int() as usize;
        current_ticks += delta;

        match midi_event.kind {
            TrackEventKind::Meta(message) => {
                match message {
                    MetaMessage::Tempo(tempo) => {
                        let tempo = 60_000_000 / tempo.as_int();

                        meta_events.push(BridgeEvent::Tempo(tempo, MidiState {
                            position_in_tick: current_ticks,
                            duration_in_tick: 0,
                            channel: 0,
                        }));
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }

    meta_events
}

pub fn bridge_notes_from_midi_track(midi_track: &MidiTrack) -> Vec<BridgeEvent> {
    let mut note_events: Vec<BridgeEvent> = Vec::new();
    let mut holding_notes: HashMap<u8, MidiNoteState> = HashMap::new();
    let mut current_ticks = 0usize;

    for midi_event in midi_track.iter() {
        let delta = midi_event.delta.as_int() as usize;
        current_ticks += delta;

        match midi_event.kind {
            TrackEventKind::Midi { channel, message } => {
                let channel = channel.as_int();

                match message {
                    MidiMessage::ProgramChange { program } => {
                        let instrument = Instrument::new(program.as_int(), channel);

                        note_events.push(BridgeEvent::ProgramChange(instrument, MidiState {
                            position_in_tick: current_ticks,
                            duration_in_tick: 0,
                            channel,
                        }));
                    },

                    MidiMessage::NoteOn { key, vel } => {
                        let key = key.as_int();
                        let vel = vel.as_int();
                        
                        if vel > 0 {
                            insert_note(
                                &mut holding_notes,
                                channel, key, vel,
                                current_ticks,
                            );
                        } else {
                            update_note(
                                &mut holding_notes,
                                &mut note_events, key,
                                current_ticks,
                            );
                        }
                    }
                    MidiMessage::NoteOff { key, .. } => {
                        let key = key.as_int();

                        update_note(
                            &mut holding_notes,
                            &mut note_events, key,
                            current_ticks,
                        );
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }

    note_events
}

fn insert_note(
    holding_notes: &mut HashMap<u8, MidiNoteState>,
    channel: u8, key: u8, velocity: u8,
    position_in_tick: usize,
) {
    holding_notes.insert(key, MidiNoteState {
        key,
        velocity,
        midi_state: MidiState {
            channel,
            position_in_tick,
            duration_in_tick: 0,
        },
    });
}

fn update_note(
    holding_notes: &mut HashMap<u8, MidiNoteState>,
    events: &mut Vec<BridgeEvent>,
    key: u8,
    position_in_tick: usize,
) {
    if let Some(mut note) = holding_notes.remove(&key) {
        let duration = position_in_tick - note.midi_state.position_in_tick;

        note.midi_state.duration_in_tick = duration;
        events.push(BridgeEvent::Note(note));
    }
}
