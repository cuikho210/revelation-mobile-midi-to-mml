use std::convert::TryInto;
use crate::{mml_event::MmlEvent, mml_track::MmlTrack, pitch_class::PitchClass, MmlSongOptions};


pub fn count_mml_note(mml_string: &String) -> usize {
    mml_string.split("&").count()
}

pub fn equalize_tracks(track_a: &mut MmlTrack, track_b: &mut MmlTrack) {
    let equalize = |a: &mut MmlTrack, b: &mut MmlTrack, gap: usize| {
        let mut mml_counter = 0usize;
        let mut index_counter = 0usize;

        for (index, event) in a.events.iter().enumerate() {
            if let MmlEvent::Note(note) = event {
                mml_counter += note.mml_note_length;

                if mml_counter >= gap {
                    index_counter = index + 1;
                    break;
                }
            }
        }
        
        let (left, right) = a.bridge_note_events.split_at(index_counter);
        let mut left = left.to_vec();

        a.bridge_note_events = right.to_vec();
        a.generate_mml_events();

        b.bridge_note_events.append(&mut left);
        a.generate_mml_events();
    };

    let length_a = track_a.mml_note_length as isize;
    let length_b = track_b.mml_note_length as isize;
    let gap = (length_a - length_b) / 2;

    if gap > 0 {
        equalize(track_a, track_b, gap as usize);
    } else {
        equalize(track_b, track_a, gap.abs() as usize);
    }
}

pub fn get_song_velocity_diff(song_options: &MmlSongOptions, tracks: &Vec<MmlTrack>) -> u8 {
    let mut velocity_max = 0u8;

    for track in tracks.iter() {
        let velocity = get_highest_velocity(&track.events);
        if velocity > velocity_max {
            velocity_max = velocity;
        }
    }

    let diff = song_options.velocity_max - velocity_max;
    diff
}

pub fn auto_boot_song_velocity(tracks: &mut Vec<MmlTrack>, velocity_diff: u8) {
    for track in tracks.iter_mut() {
        track.apply_boot_velocity(velocity_diff);
    }
}

pub fn count_mml_notes(mml: &String) -> usize {
    mml.split('&').count()
}

pub fn midi_velocity_to_mml_velocity(
    midi_velocity: u8,
    velocity_min: u8,
    velocity_max: u8,
) -> u8 {
    let range: i32 = (velocity_max - velocity_min).into();
    let midi_velocity: i32 = midi_velocity.into();
    let velocity_min: i32 = velocity_min.into();

    ((midi_velocity * range / 127) + velocity_min).try_into().unwrap()
}

pub fn get_highest_velocity(events: &Vec<MmlEvent>) -> u8 {
    let mut max = 0u8;

    for event in events.iter() {
        if let MmlEvent::Velocity(vel) = event {
            if *vel > max {
                max = *vel;
            }
        }
    }

    max
}

pub fn midi_key_to_pitch_class(midi_key: u8) -> PitchClass {
    let classes: [PitchClass; 12] = [
        PitchClass::C,
        PitchClass::Db,
        PitchClass::D,
        PitchClass::Eb,
        PitchClass::E,
        PitchClass::F,
        PitchClass::Gb,
        PitchClass::G,
        PitchClass::Ab,
        PitchClass::A,
        PitchClass::Bb,
        PitchClass::B,
    ];
    let index = midi_key % 12;
    classes[index as usize].to_owned()
}

pub fn midi_key_to_octave(midi_key: u8) -> u8 {
    (midi_key / 12) - 1
}

pub fn get_smallest_unit_in_tick(ppq: u16, smallest_unit: usize) -> f32 {
    ppq as f32 / (smallest_unit as f32 / 4.)
}

pub fn tick_to_smallest_unit(tick: usize, ppq: u16, smallest_unit: usize) -> usize {
    let note = get_smallest_unit_in_tick(ppq, smallest_unit);
    let duration_in_note = tick as f32 / note;

    duration_in_note.round() as usize
}

#[derive(Debug, Clone, PartialEq)]
struct CustomMmlNote {
    duration_in_smallest_unit: usize,
    mml_value: usize,
}

impl CustomMmlNote {
    pub fn new(smallest_unit: usize, duration_in_smallest_unit: usize) -> Self {
        Self {
            duration_in_smallest_unit,
            mml_value: smallest_unit / duration_in_smallest_unit,
        }
    }
}

fn get_list_of_mml_notes(smallest_unit: usize) -> Vec<CustomMmlNote> {
    let mut notes: Vec<CustomMmlNote> = Vec::new();
    let mut remainder = smallest_unit;

    while remainder > 1 {
        notes.push(CustomMmlNote::new(smallest_unit, remainder));
        remainder = remainder / 2;
    }
    notes.push(CustomMmlNote::new(smallest_unit, remainder));

    notes
}

pub fn get_display_mml(mut duration_in_smallest_unit: usize, note_class: &PitchClass, smallest_unit: usize) -> String {
    let mut result: Vec<String> = Vec::new();
    let notes = get_list_of_mml_notes(smallest_unit);

    while duration_in_smallest_unit > 0 {
        let mut current_note: usize = 0;

        for mml_note in notes.iter() {
            if duration_in_smallest_unit >= mml_note.duration_in_smallest_unit {
                duration_in_smallest_unit -= mml_note.duration_in_smallest_unit;
                current_note = mml_note.mml_value;
                break;
            }
        }

        result.push(format!("{}{}", note_class, current_note));

        let half_of_current_note = smallest_unit / (current_note * 2);
        if duration_in_smallest_unit > 0 && duration_in_smallest_unit >= half_of_current_note {
            result.push(".".to_string());
            duration_in_smallest_unit -= half_of_current_note;
        }

        if duration_in_smallest_unit == 0 {
            break;
        } else {
            result.push("&".to_string());
        }
    }

    result.join("")
}
