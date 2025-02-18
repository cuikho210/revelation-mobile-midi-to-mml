use crate::{NoteEvent, SynthOutputConnection};
use std::time::Duration;

const SMALLEST_UNIT: usize = 256;

pub fn mml_velocity_to_midi_velocity(mml_velocity: u8) -> u8 {
    let mml_f64: f64 = mml_velocity as f64;
    (mml_f64 / 15.0 * 127.0) as u8
}

pub fn mml_to_midi_key(mml: &str, octave: u8) -> Option<u8> {
    let mml = mml.to_lowercase();
    let chars = mml.chars();
    let mut midi_key: i8 = -1;

    for char in chars {
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

pub fn mml_duration_to_duration_in_smallest_unit(mml_duration: &str) -> usize {
    let mut is_has_a_dot = false;
    let mut mml = mml_duration;
    let last = mml.chars().last().unwrap();

    if last == '.' {
        mml = &mml[..mml.len() - 1];
        is_has_a_dot = true;
    }

    let mml_duration = mml.parse::<usize>().unwrap();
    let mut result = SMALLEST_UNIT / mml_duration;

    if is_has_a_dot {
        result += result / 2;
    }

    result
}

pub fn duration_in_smallest_unit_to_ms(duration_in_smallest_unit: usize, tempo: usize) -> usize {
    let tempo_f64 = tempo as f64;
    let smallest_unit_f64 = SMALLEST_UNIT as f64;
    let dur_per_smallest_unit_in_ms = 240000.0 / (smallest_unit_f64 * tempo_f64);

    let result = duration_in_smallest_unit as f64 * dur_per_smallest_unit_in_ms;
    result.round() as usize
}

pub fn play_note(
    mut connection: SynthOutputConnection,
    note: &NoteEvent,
    channel: u8,
    duration: Option<Duration>,
) -> Duration {
    println!("Play note {}", note.raw_mml);
    let duration = match duration {
        Some(value) => value,
        None => Duration::from_millis(note.duration_in_ms as u64),
    };

    if let Some(key) = note.midi_key {
        connection.note_on(channel, key, note.midi_velocity);
    }

    duration
}

pub fn stop_note(mut connection: SynthOutputConnection, note: &NoteEvent, channel: u8) {
    if let Some(key) = note.midi_key {
        connection.note_off(channel, key);
    }
}

pub fn stop_chord(mut connection: SynthOutputConnection, chord: &[NoteEvent], channel: u8) {
    for note in chord.iter() {
        if let Some(key) = note.midi_key {
            connection.note_off(channel, key);
        }
    }
}

pub fn play_chord(
    mut connection: SynthOutputConnection,
    chord: &Vec<NoteEvent>,
    channel: u8,
    duration: Option<Duration>,
) -> Duration {
    println!("Play chord {}", chord.first().unwrap().raw_mml);

    let duration = match duration {
        Some(value) => value,
        None => Duration::from_millis(get_longest_note_duration(chord) as u64),
    };

    for note in chord.iter() {
        if let Some(key) = note.midi_key {
            connection.note_on(channel, key, note.midi_velocity);
        }
    }

    duration
}

pub fn get_longest_note_duration(notes: &Vec<NoteEvent>) -> isize {
    let mut max: isize = 0;

    for note in notes {
        let duration = note.duration_in_ms as isize;

        if duration > max {
            max = duration;
        }
    }

    max
}
