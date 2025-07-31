#![allow(dead_code)]

use midi_to_mml::{BridgeEvent, MmlEvent, MmlTrack};

pub trait TrackTestExt {
    /// by tick
    fn get_bridge_events_duration(&self) -> usize;

    /// by smallest_unit
    fn get_mml_events_duration(&self) -> usize;
}

impl TrackTestExt for MmlTrack {
    fn get_bridge_events_duration(&self) -> usize {
        self.bridge_events
            .iter()
            .map(|e| {
                if let BridgeEvent::Note(note) = e {
                    note.midi_state.position_in_tick + note.midi_state.duration_in_tick
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0)
    }

    fn get_mml_events_duration(&self) -> usize {
        self.events
            .iter()
            .map(|e| match e {
                MmlEvent::Note(note) => {
                    if note.is_part_of_chord {
                        0
                    } else {
                        note.duration_in_smallest_unit
                    }
                }
                MmlEvent::Rest(dur) => *dur,
                _ => 0,
            })
            .sum()
    }
}
