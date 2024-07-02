use crate::utils;

pub struct NoteEvent {
    pub raw_mml: String,
    pub midi_key: Option<u8>,
    pub midi_velocity: u8,
    pub duration_in_note_64: usize,
}

impl NoteEvent {
    pub fn from_mml(mml: String, octave: u8, velocity: u8, tempo: u16) -> Self {
        let mut parts = mml.split('&');
        let mut mml_key: Option<String> = None;
        let mut key_length = 1usize;
        let mut midi_key: Option<u8> = None;
        let mut duration_in_note_64: usize = 0;
        let midi_velocity = utils::mml_velocity_to_midi_velocity(velocity);
        
        while let Some(part) = parts.next() {
            if let None = mml_key {
                let mml_key_value = utils::get_mml_key(part);

                key_length = mml_key_value.len();
                midi_key = utils::mml_to_midi_key(&mml_key_value, octave);
                mml_key = Some(mml_key_value);
            }

            let duration_part = &part[key_length..];
            let duration = utils::mml_duration_to_duration_in_note_64(duration_part);
            duration_in_note_64 += duration;
        }

        Self {
            raw_mml: mml,
            midi_key,
            midi_velocity,
            duration_in_note_64,
        }
    }
}
