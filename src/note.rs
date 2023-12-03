use crate::utils;

#[derive(Debug, Clone)]
pub struct Note {
    pub pitch_class: String,
    pub octave: u8,

    pub duration_in_tick: u32,
    pub position_in_tick: u32,
}

impl Note {
    pub fn new(midi_key: u8, current_tick: u32) -> Self {
        let pitch_class = utils::midi_key_to_pitch_class(midi_key);
        let octave = utils::midi_key_to_octave(midi_key);
        
        Self {
            pitch_class,
            octave,
            position_in_tick: current_tick,
            duration_in_tick: 0,
        }
    }

    pub fn to_mml(&self, ppq: &u16) -> String {
        utils::get_display_mml(ppq, self.duration_in_tick, &self.pitch_class)
    }
}