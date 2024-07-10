use std::convert::TryInto;
use crate::{
    mml_note::MmlNote,
    mml_event::MmlEvent,
    pitch_class::PitchClass,
};

// Note 64. Is whole_note/64 or quarter_note/16
const SMALLEST_UNIT: usize = 64;

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

pub fn get_highest_velocity(notes: &Vec<MmlNote>) -> u8 {
    let mut max = 0u8;

    for note in notes.iter() {
        if note.velocity > max {
            max = note.velocity;
        }
    }

    max
}

/// When a NoteOn event is emitted while another note is playing,
/// it must either be joined with the previous note to form a chord,
/// or the previous note must be cut short.
/// This is because MML code can only play one note or chord at a time.
pub fn try_connect_to_chord(
    events: &mut Vec<MmlEvent>,
    current_note: &MmlNote,
    before_note: &MmlNote,
) -> bool {
    return if is_can_connect_to_chord(current_note, before_note) {
        events.push(MmlEvent::ConnectChord);
        true
    } else {
        false
    };
}

pub fn is_can_connect_to_chord(current_note: &MmlNote, before_note: &MmlNote) -> bool {
    let position_diff =
        current_note.position_in_smallest_unit - before_note.position_in_smallest_unit;
    let is_same_position = position_diff < 1;

    is_same_position
}

/// Cut the duration of all previous notes by a position in ticks
pub fn cut_previous_notes(events: &mut Vec<MmlEvent>, position: usize) {
    for event in events.iter_mut() {
        if let MmlEvent::Note(note) = event {
            let note_end_position = note.position_in_smallest_unit + note.duration_in_smallest_unit;

            if note_end_position > position {
                let position_diff = note_end_position - position;
                let duration = note.duration_in_smallest_unit - position_diff;
                note.duration_in_smallest_unit = duration;
            }
        }
    }
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

pub fn get_smallest_unit_in_tick(ppq: u16) -> f32 {
    ppq as f32 / (SMALLEST_UNIT as f32 / 4.)
}

pub fn tick_to_smallest_unit(tick: usize, ppq: u16) -> usize {
    let note = get_smallest_unit_in_tick(ppq);
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

pub fn get_display_mml(mut duration_in_smallest_unit: usize, note_class: &PitchClass) -> String {
    let mut result: Vec<String> = Vec::new();
    let notes = get_list_of_mml_notes(SMALLEST_UNIT.to_owned());

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

        let half_of_current_note = SMALLEST_UNIT / (current_note * 2);
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
