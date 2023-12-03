

pub fn midi_key_to_pitch_class(midi_key: u8) -> String {
    let classes: [&str; 12] = ["C", "C+", "D", "D+", "E", "F", "F+", "G", "G+", "A", "A+", "B"];
    let index = midi_key % 12;
    classes[index as usize].to_string()
}

pub fn midi_key_to_octave(midi_key: u8) -> u8 {
    (midi_key / 12) - 1
}

pub fn tick_to_note_64(tick: u32, ppq: u16) -> u32 {
    let note: f32 = ppq as f32 / 16.;
    let duration_in_note = tick as f32 / note;

    duration_in_note.round() as u32
}

struct Note {
    duration_in_note_64: u32,
    mml_value: u32,
}

impl Note {
    pub fn new(duration_in_note_64: u32) -> Self {
        Note {
            duration_in_note_64,
            mml_value: 64 / duration_in_note_64,
        }
    }
}

pub fn get_display_mml(ppq: &u16, duration_in_tick: u32, note_class: &String) -> String {
    let mut result: Vec<String> = Vec::new();
    let mut main_note: Option<u32> = None;
    let mut duration_in_note_64 = tick_to_note_64(duration_in_tick, ppq.to_owned());

    let notes: [Note;7] = [
        // Whole note
        Note::new(64),
        // Half note
        Note::new(32),
        // Quarter note
        Note::new(16),
        // Eighth note
        Note::new(8),
        // Sixteenth note
        Note::new(4),
        // Thirty second note
        Note::new(2),
        // Sixty fourth note
        Note::new(1),
    ];

    while duration_in_note_64 > 0 {
        let mut value: u32 = 0;

        for note in notes.iter() {
            if duration_in_note_64 >= note.duration_in_note_64 {
                duration_in_note_64 -= note.duration_in_note_64;
                value = note.mml_value;
                break;
            }
        }

        result.push(format!("{}{}", note_class, value));

        if let None = main_note {
            main_note = Some(value);
        }

        if let Some(main_note) = main_note {
            let half_of_main_note = 64 / (main_note * 2);

            if duration_in_note_64 >= half_of_main_note {
                result.push(".".to_string());
                duration_in_note_64 -= half_of_main_note;
            }
        }

        if duration_in_note_64 == 0 {
            break;
        } else {
            result.push("&".to_string());
        }
    }

    result.join("")
}

#[cfg(test)]
mod test {
    #[test]
    fn test_display_mml() {

    }
}