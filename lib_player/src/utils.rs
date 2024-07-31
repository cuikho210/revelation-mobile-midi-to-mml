use std::{thread::sleep, time::Duration};
use crate::{SynthOutputConnection, NoteEvent};

const SMALLEST_UNIT: usize = 256;

pub fn mml_velocity_to_midi_velocity(mml_velocity: u8) -> u8 {
    let mml_f64: f64 = mml_velocity as f64;
    (mml_f64 / 15.0 * 127.0) as u8
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

pub fn play_note(connection: &mut SynthOutputConnection, note: &NoteEvent, channel: u8, duration: Option<Duration>) {
    let duration = match duration {
        Some(value) => value,
        None => Duration::from_millis(note.duration_in_ms as u64),
    };

    if let Some(key) = note.midi_key {
        log_note_on(note, channel);
        connection.note_on(channel, key, note.midi_velocity);

        sleep(duration);
        connection.note_off(channel, key);
    } else {
        sleep(duration);
    }
}

pub fn play_chord(connection: &mut SynthOutputConnection, chord: &Vec<NoteEvent>, channel: u8, duration: Option<Duration>) {
    let duration = match duration {
        Some(value) => value,
        None => Duration::from_millis(chord.first().unwrap().duration_in_ms as u64),
    };

    for note in chord.iter() {
        log_note_on(note, channel);
        if let Some(key) = note.midi_key {
            connection.note_on(channel, key, note.midi_velocity);
        }
    }

    sleep(duration);

    for note in chord.iter() {
        if let Some(key) = note.midi_key {
            connection.note_off(channel, key);
        }
    }
}

pub fn log_note_on(note: &NoteEvent, channel: u8) {
    let midi_key = match note.midi_key {
        Some(key) => key.to_string(),
        None => String::from("rest"),
    };

    println!(
        "[play_note] note_on {} {} - velocity {} - duration {}ms - channel {}",
        midi_key,
        note.raw_mml,
        note.midi_velocity,
        note.duration_in_ms,
        channel,
    );
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
