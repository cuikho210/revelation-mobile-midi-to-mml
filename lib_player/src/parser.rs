use crate::{mml_event::MmlEvent, note_event::NoteEvent};

const NOTE_NAMES: [char; 8] = ['c', 'd', 'e', 'f', 'g', 'a', 'b', 'r'];
const NOTE_EXTRAS: [char; 3] = ['&', '.', '+'];

#[derive(Debug, Clone)]
pub struct Parser {
    pub index: usize,
    pub raw_mml: String,
    pub notes: Vec<NoteEvent>,
}

impl Parser {
    pub fn parse(index: usize, mml: String) -> Self {
        let mut result = Self {
            index,
            raw_mml: mml,
            notes: Vec::new(),
        };

        result.notes = parse_note_events(&result.raw_mml);
        result
    }
}

fn parse_note_events(mml: &str) -> Vec<NoteEvent> {
    let mut index = 0usize;
    let mut current_mml_velocity = 12u8;
    let mut current_octave = 4u8;
    let mut current_tempo = 120usize;
    let mut is_connect_chord = false;
    let mut notes: Vec<NoteEvent> = Vec::new();

    while let Some(event) = parse_event(
        mml,
        &mut index,
        current_mml_velocity,
        current_octave,
        current_tempo,
        &mut is_connect_chord,
    ) {
        match event {
            MmlEvent::SetNote(note) => notes.push(note),
            MmlEvent::SetTempo(tempo) => current_tempo = tempo,
            MmlEvent::SetOctave(octave) => current_octave = octave,
            MmlEvent::IncreOctave => current_octave += 1,
            MmlEvent::DecreOctave => {
                current_octave = current_octave.saturating_sub(1);
            }
            MmlEvent::SetVelocity(velocity) => current_mml_velocity = velocity,
            MmlEvent::ConnectChord => is_connect_chord = true,
            MmlEvent::Empty => (),
        }
    }

    notes
}

fn parse_event(
    raw_mml: &str,
    index: &mut usize,
    current_mml_velocity: u8,
    current_mml_octave: u8,
    current_tempo: usize,
    is_connect_chord: &mut bool,
) -> Option<MmlEvent> {
    match raw_mml.chars().nth(*index) {
        Some(char) => {
            let mml = &raw_mml[*index..];

            if char == 't' {
                let value = get_first_mml_value(mml);
                *index += value.len() + 1;

                let tempo = value.parse::<usize>().unwrap();

                Some(MmlEvent::SetTempo(tempo))
            } else if char == 'o' {
                let value = &mml[1..2];
                *index += 2;

                let octave = value.parse::<u8>().unwrap();

                Some(MmlEvent::SetOctave(octave))
            } else if char == 'v' {
                let value = get_first_mml_value(mml);
                *index += value.len() + 1;

                let velocity = value.parse::<u8>().unwrap();

                Some(MmlEvent::SetVelocity(velocity))
            } else if char == '>' {
                *index += 1;

                Some(MmlEvent::IncreOctave)
            } else if char == '<' {
                *index += 1;

                Some(MmlEvent::DecreOctave)
            } else if char == ':' {
                *index += 1;

                Some(MmlEvent::ConnectChord)
            } else if NOTE_NAMES.contains(&char) {
                let mml_note = get_first_mml_note(mml);
                let mml_note_length = mml_note.len();

                let note = NoteEvent::from_mml(
                    mml_note,
                    current_mml_octave,
                    current_mml_velocity,
                    current_tempo,
                    *is_connect_chord,
                    *index,
                );

                *is_connect_chord = false;
                *index += mml_note_length;

                Some(MmlEvent::SetNote(note))
            } else {
                *index += 1;
                Some(MmlEvent::Empty)
            }
        }
        None => None,
    }
}

fn get_first_mml_note(mml: &str) -> String {
    let mut chars = mml.chars();
    let mut result = String::new();
    let mut is_note_extra_checked = false;
    let mut before_char = chars.next().unwrap();
    let note_name = before_char;

    result.push(note_name);

    for char in chars {
        if !is_note_extra_checked {
            if char == '+' {
                result.push(char);
                continue;
            }

            is_note_extra_checked = true;
        }

        let mut is_break = true;

        if char.is_ascii_digit() || NOTE_EXTRAS.contains(&char) {
            is_break = false;
        }

        if is_break && (char == note_name && before_char == '&') {
            is_break = false;
        }

        if is_break {
            break;
        } else {
            before_char = char;
            result.push(char);
        }
    }

    result
}

fn get_first_mml_value(mml: &str) -> String {
    let chars = mml[1..].chars();
    let mut result = String::new();

    for char in chars {
        if char.is_ascii_digit() {
            result.push(char);
        } else {
            break;
        }
    }

    result
}
