use crate::utils;

#[derive(Debug, Clone)]
pub struct NoteEvent {
    pub raw_mml: String,
    pub tempo: usize,
    pub midi_key: Option<u8>,
    pub midi_velocity: u8,
    pub duration_in_note_64: usize,
    pub duration_in_ms: usize,
    pub is_connected_to_prev_note: bool,
}

impl NoteEvent {
    pub fn from_mml(
        mml: String,
        octave: u8,
        velocity: u8,
        tempo: usize,
        is_connected_to_prev_note: bool,
    ) -> Self {
        let mut parts = mml.split('&');
        let mut mml_key: Option<String> = None;
        let mut key_length: usize = 1;
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

        let duration_in_ms: usize = utils::duration_in_note_64_to_ms(duration_in_note_64, tempo);

        Self {
            raw_mml: mml,
            tempo,
            midi_key,
            midi_velocity,
            duration_in_note_64,
            duration_in_ms,
            is_connected_to_prev_note,
        }
    }
}