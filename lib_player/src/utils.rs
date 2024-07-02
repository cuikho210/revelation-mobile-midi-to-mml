
pub fn mml_velocity_to_midi_velocity(mml_velocity: u8) -> u8 {
    mml_velocity / 15 * 128
}

pub fn mml_to_midi_key(mml: &str, octave: u8) -> Option<u8> {
    let mml = mml.to_lowercase();
    let mut chars = mml.chars();
    let mut midi_key: i8 = -1;

    while let Some(char) = chars.next() {
        match char {
            'c' => midi_key = 12,
            'd' => midi_key = 14,
            'e' => midi_key = 16,
            'f' => midi_key = 17,
            'g' => midi_key = 19,
            'a' => midi_key = 21,
            'b' => midi_key = 23,
            '+' => midi_key += 1,
            _ => (),
        };
    }

    if midi_key < 0 {
        return None;
    }

    let mut result: u8 = midi_key.try_into().unwrap();
    result += octave * 12;
    Some(result)
}

/// `c+4&c+32.` => `c+`
pub fn get_mml_key(mml: &str) -> String {
    let mut chars = mml.chars();
    let first = chars.next().unwrap();
    let second = chars.next().unwrap();

    if second == '+' {
        return format!("{}{}", first, second);
    }

    first.to_string()
}

pub fn mml_duration_to_duration_in_note_64(mml_duration: &str) -> usize {
    let mut is_has_a_dot = false;
    let mut mml = mml_duration;
    let last = mml.chars().last().unwrap();

    if last == '.' {
        mml = &mml[..mml.len() - 1];
        is_has_a_dot = true;
    }

    let mml_duration = mml.parse::<usize>().unwrap();
    let mut result = 64 / mml_duration;
    
    if is_has_a_dot {
        result += result / 2;
    }

    result
}
