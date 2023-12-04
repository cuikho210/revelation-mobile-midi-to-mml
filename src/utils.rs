use crate::note::Note;
use crate::track_event::TrackEvent;

// Note 64. Is whole_note/64 or quarter_note/16
const SMALLEST_UNIT: u32 = 64;

/// When a NoteOn event is emitted while another note is playing,
/// it must either be joined with the previous note to form a chord,
/// or the previous note must be cut short.
/// This is because MML code can only play one note or chord at a time.
pub fn connect_to_chord_or_cut_before_note(events: &mut Vec<TrackEvent>, ppq: u16, current_note: &Note, before_note: &mut Note) {
    let position_diff = current_note.position_in_tick - before_note.position_in_tick;
    let smallest_unit = get_smallest_unit_in_tick(ppq).round() as u32;

    let is_same_duration = current_note.duration_in_tick > before_note.duration_in_tick / 5;
    let is_same_position = position_diff < before_note.duration_in_tick / 5;
    let is_lesser_smallest_unit = position_diff < smallest_unit;

    if is_lesser_smallest_unit || (is_same_position && is_same_duration) {
        events.push(TrackEvent::ConnectChord);
    } else {
        before_note.duration_in_tick = position_diff - 1;
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

#[derive(Debug, Clone)]
struct MMLNote {
    duration_in_smallest_unit: u32,
    mml_value: u32,
}

impl MMLNote {
    pub fn new(duration_in_smallest_unit: u32) -> Self {
        Self {
            duration_in_smallest_unit,
            mml_value: SMALLEST_UNIT / duration_in_smallest_unit,
        }
    }
}

pub fn get_display_mml(ppq: &u16, duration_in_tick: u32, note_class: &String) -> String {
    let mut result: Vec<String> = Vec::new();
    let mut main_note: Option<u32> = None;
    let mut duration_in_smallest_unit = tick_to_smallest_unit(duration_in_tick, ppq.to_owned());

    let mut notes: Vec<MMLNote> = Vec::new();
    let mut smallest_unit = SMALLEST_UNIT.to_owned();

    while smallest_unit > 1 {
        notes.push(MMLNote::new(smallest_unit));
        smallest_unit = smallest_unit / 2;
    }
    notes.push(MMLNote::new(smallest_unit));

    while duration_in_smallest_unit > 0 {
        let mut value: u32 = 0;

        for mml_note in notes.iter() {
            if duration_in_smallest_unit >= mml_note.duration_in_smallest_unit {
                duration_in_smallest_unit -= mml_note.duration_in_smallest_unit;
                value = mml_note.mml_value;
                break;
            }
        }

        result.push(format!("{}{}", note_class, value));

        if let None = main_note {
            main_note = Some(value);
        }

        if let Some(main_note) = main_note {
            let half_of_main_note = SMALLEST_UNIT / (main_note * 2);

            if duration_in_smallest_unit >= half_of_main_note {
                result.push(".".to_string());
                duration_in_smallest_unit -= half_of_main_note;
            }
        }

        if duration_in_smallest_unit == 0 {
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