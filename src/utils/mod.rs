#[cfg(test)]
mod test;

use crate::note::Note;
use crate::track_event::TrackEvent;

// Note 64. Is whole_note/64 or quarter_note/16
const SMALLEST_UNIT: u32 = 64;

/// When a NoteOn event is emitted while another note is playing,
/// it must either be joined with the previous note to form a chord,
/// or the previous note must be cut short.
/// This is because MML code can only play one note or chord at a time.
pub fn try_connect_to_chord(
    events: &mut Vec<TrackEvent>,
    current_note: &Note,
    before_note: &Note,
) -> bool {
    let position_diff = current_note.position_in_smallest_unit - before_note.position_in_smallest_unit;
    let is_same_duration = current_note.duration_in_smallest_unit > before_note.duration_in_smallest_unit / 5;
    let is_same_position = position_diff < before_note.duration_in_tick / 5;
    let is_less_than_smallest_unit = position_diff < 1;

    return if is_less_than_smallest_unit || is_same_position && is_same_duration {
        events.push(TrackEvent::ConnectChord);
        true
    } else {
        false
    }
}

/// Cut the duration of all previous notes by a position in ticks
pub fn cut_previous_notes(
    events: &mut Vec<TrackEvent>,
    position: u32,
) {
    for event in events.iter_mut() {
        if let TrackEvent::SetNote(note) = event {
            let note_end_position = note.position_in_smallest_unit + note.duration_in_smallest_unit;

            if note_end_position > position {
                let position_diff: u32 = note_end_position - position;
                let duration = note.duration_in_smallest_unit - position_diff;
                note.duration_in_smallest_unit = duration;
            }
        }
    }
}

pub fn midi_key_to_pitch_class(midi_key: u8) -> String {
    let classes: [&str; 12] = ["C", "C+", "D", "D+", "E", "F", "F+", "G", "G+", "A", "A+", "B"];
    let index = midi_key % 12;
    classes[index as usize].to_string()
}

pub fn midi_key_to_octave(midi_key: u8) -> u8 {
    (midi_key / 12) - 1
}

pub fn get_smallest_unit_in_tick(ppq: u16) -> f32 {
    ppq as f32 / (SMALLEST_UNIT as f32 / 4.)
}

pub fn tick_to_smallest_unit(tick: u32, ppq: u16) -> u32 {
    let note = get_smallest_unit_in_tick(ppq);
    let duration_in_note = tick as f32 / note;

    duration_in_note.round() as u32
}

#[derive(Debug, Clone, PartialEq)]
struct MMLNote {
    duration_in_smallest_unit: u32,
    mml_value: u32,
}

impl MMLNote {
    pub fn new(smallest_unit: u32, duration_in_smallest_unit: u32) -> Self {
        Self {
            duration_in_smallest_unit,
            mml_value: smallest_unit / duration_in_smallest_unit,
        }
    }
}

fn get_list_of_mml_notes(smallest_unit: u32) -> Vec<MMLNote> {
    let mut notes: Vec<MMLNote> = Vec::new();
    let mut remainder = smallest_unit;

    while remainder > 1 {
        notes.push(MMLNote::new(smallest_unit, remainder));
        remainder = remainder / 2;
    }
    notes.push(MMLNote::new(smallest_unit, remainder));

    notes
}

pub fn get_display_mml(mut duration_in_smallest_unit: u32, note_class: &String) -> String {
    let mut result: Vec<String> = Vec::new();
    let notes = get_list_of_mml_notes(SMALLEST_UNIT.to_owned());

    while duration_in_smallest_unit > 0 {
        let mut current_note: u32 = 0;

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
