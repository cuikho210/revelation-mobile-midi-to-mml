use crate::utils;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub pitch_class: String,
    pub octave: u8,

    pub duration_in_tick: u32,
    pub position_in_tick: u32,
    pub duration_in_smallest_unit: u32,
    pub position_in_smallest_unit: u32,
}

impl Note {
    pub fn new(ppq: u16, midi_key: u8, current_tick: u32) -> Self {
        let pitch_class = utils::midi_key_to_pitch_class(midi_key);
        let octave = utils::midi_key_to_octave(midi_key);
        let position_in_smallest_unit = utils::tick_to_smallest_unit(current_tick, ppq);
        
        Self {
            pitch_class,
            octave,
            position_in_tick: current_tick,
            position_in_smallest_unit,
            duration_in_tick: 0,
            duration_in_smallest_unit: 0,
        }
    }

    pub fn to_mml(&self) -> String {
        utils::get_display_mml(self.duration_in_smallest_unit, &self.pitch_class)
    }
}
