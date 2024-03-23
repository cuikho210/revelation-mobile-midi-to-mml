use std::collections::HashMap;
use crate::{
    note::Note,
    utils,
};
use midly::{MidiMessage, TrackEventKind};

pub fn get_notes_from_smf_track(
    smf_track: &midly::Track,
    ppq: u16,
    velocity_min: u8,
    velocity_max: u8,
) -> Vec<Note> {
    let mut result: Vec<Note> = Vec::new();
    let mut holding_notes: HashMap<u8, usize> = HashMap::new();
    let mut current_ticks = 0u32;

    for midi_event in smf_track.iter() {
        let delta = midi_event.delta.as_int();
        current_ticks += delta;

        match midi_event.kind {
            TrackEventKind::Midi { message, channel } => match message {
                MidiMessage::NoteOn { key, vel } => {
                    let midi_key = key.as_int();
                    let midi_velocity = vel.as_int();
                    let mml_velocity = utils::midi_velocity_to_mml_velocity(midi_velocity, velocity_min, velocity_max);
                    let midi_channel = channel.as_int() + 1;

                    if vel.as_int() > 0 {
                        create_note(
                            midi_channel,
                            midi_key,
                            midi_velocity,
                            mml_velocity,
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
                _ => (),
            },
            _ => (),
        }
    }

    result
}

pub fn create_note(
    midi_channel: u8,
    midi_key: u8,
    midi_velocity: u8,
    mml_velocity: u8,
    current_ticks: u32,
    notes: &mut Vec<Note>,
    holding_notes: &mut HashMap<u8, usize>,
    ppq: u16,
) {
    let note = Note::new(
        ppq,
        midi_channel,
        midi_key,
        midi_velocity,
        mml_velocity,
        current_ticks,
    );

    holding_notes.insert(midi_key, notes.len());
    notes.push(note);
}

pub fn update_note(
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
            note.update_mml_string();
        }
    }
}
