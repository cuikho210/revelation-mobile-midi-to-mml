
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch_class: String,
    pub octave: u8,

    /// `duration` is the number of parts compared to a whole note.  
    /// Example a quarter note is 4, a sixteenth note is 16, ...
    pub duration: u8,

    pub duration_in_note_64: u16,
    pub position_in_note_64: u16,
    pub duration_in_tick: u32,
    pub position_in_tick: u32,
}

impl Note {
    pub fn new(midi_key: u8, current_tick: u32, ppq: &u16) -> Self {
        let pitch_class = Self::midi_key_to_pitch_class(midi_key);
        let octave = Self::midi_key_to_octave(midi_key);
        let position_in_note_64 = Self::tick_to_note_64(current_tick, ppq.to_owned());
        
        Self {
            pitch_class,
            octave,
            position_in_note_64,
            position_in_tick: current_tick,
            duration: 0,
            duration_in_note_64: 0,
            duration_in_tick: 0,
        }
    }

    pub fn get_duration(duration_in_note_64: u16) -> u8 {
        (64 / duration_in_note_64) as u8
    }

    pub fn tick_to_note_64(tick: u32, ppq: u16) -> u16 {
        (tick / (ppq as u32 / 16)) as u16
    }

    pub fn to_mml(&self) -> String {
        if self.duration == 0 {
            return String::new();
        }

        format!(
            "{}{}",
            self.pitch_class,
            self.duration,
        )
    }

    fn midi_key_to_pitch_class(midi_key: u8) -> String {
        let classes: [&str; 12] = ["C", "C+", "D", "D+", "E", "F", "F+", "G", "G+", "A", "A+", "B"];
        let index = midi_key % 12;
        classes[index as usize].to_string()
    }

    fn midi_key_to_octave(midi_key: u8) -> u8 {
        (midi_key / 12) - 1
    }

}