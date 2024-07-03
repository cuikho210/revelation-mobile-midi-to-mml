use std::time::{Duration, Instant};

use crate::{
    mml_event::MmlEvent,
    note_event::NoteEvent, SynthOutputConnection,
    utils,
};

pub struct Parser {
    pub raw_mml: String,
    pub notes: Vec<NoteEvent>,
    pub duration_in_ms: usize,
}

impl Parser {
    pub fn parse(mml: String) -> Self {
        let mut result = Self {
            raw_mml: mml,
            notes: Vec::new(),
            duration_in_ms: 0,
        };

        result.parse_note_events();
        result
    }

    pub fn play(&self, connection: &mut SynthOutputConnection, channel: u8) {
        let mut before: Option<NoteEvent> = None;
        let mut current_chord: Vec<NoteEvent> = Vec::new();
        let mut absolute_duration: isize = 0;
        let time = Instant::now();

        for note in self.notes.iter() {
            if note.is_connected_to_prev_note {
                if let Some(before_note) = before.as_ref() {
                    if current_chord.len() == 0 {
                        current_chord.push(before_note.to_owned());
                    }
                }

                current_chord.push(note.to_owned());
                continue;
            }

            let correct_duration = time.elapsed().as_millis() as isize;
            let duration_diff = correct_duration - absolute_duration;

            if current_chord.len() > 0 {
                let duration_isize = current_chord.first().unwrap().duration_in_ms as isize;
                let duration = duration_isize - duration_diff;
                let duration = Duration::from_millis(duration as u64);

                utils::play_chord(
                    connection,
                    &current_chord,
                    channel,
                    Some(duration),
                );

                absolute_duration += duration_isize;
                current_chord.clear();
                before = Some(note.to_owned());
                continue;
            }

            if let Some(before_note) = &before {
                let duration_isize = before_note.duration_in_ms as isize;
                let duration = duration_isize - duration_diff;
                let duration = Duration::from_millis(duration as u64);

                utils::play_note(
                    connection,
                    before_note,
                    channel,
                    Some(duration),
                );

                absolute_duration += duration_isize;
            }

            before = Some(note.to_owned());
        }
    }

    fn parse_note_events(&mut self) {
        let mut index = 0usize;
        let mut current_mml_velocity = 12u8;
        let mut current_octave = 4u8;
        let mut current_tempo = 120usize;
        let mut is_connect_chord = false;

        loop {
            if let Some(event) = self.parse_event(
                &mut index,
                current_mml_velocity,
                current_octave,
                current_tempo,
                &mut is_connect_chord,
            ) {
                match event {
                    MmlEvent::SetNote(note) => {
                        if note.is_connected_to_prev_note == false {
                            self.duration_in_ms += note.duration_in_ms;
                        }

                        self.notes.push(note);
                    },
                    MmlEvent::SetTempo(tempo) => current_tempo = tempo,
                    MmlEvent::SetOctave(octave) => current_octave = octave,
                    MmlEvent::IncreOctave => current_octave += 1,
                    MmlEvent::DecreOctave => current_octave -= 1,
                    MmlEvent::SetVelocity(velocity) => current_mml_velocity = velocity,
                    MmlEvent::ConnectChord => is_connect_chord = true,
                }
            } else {
                break;
            }
        }
    }

    fn parse_event(
        &self,
        index: &mut usize,
        current_mml_velocity: u8,
        current_mml_octave: u8,
        current_tempo: usize,
        is_connect_chord: &mut bool,
    ) -> Option<MmlEvent> {
        match self.raw_mml.chars().nth(*index) {
            Some(char) => {
                let mml = &self.raw_mml.as_str()[*index..];

                if char == 't' {
                    let value = get_first_mml_value(mml);
                    *index += value.len() + 1;

                    let tempo = value.parse::<usize>().unwrap();
                    return Some(MmlEvent::SetTempo(tempo));
                } else if char == 'o' {
                    let value = get_first_mml_value(mml);
                    *index += value.len() + 1;

                    let octave = value.parse::<u8>().unwrap();
                    return Some(MmlEvent::SetOctave(octave));
                } else if char == 'v' {
                    let value = get_first_mml_value(mml);
                    *index += value.len() + 1;

                    let velocity = value.parse::<u8>().unwrap();
                    return Some(MmlEvent::SetVelocity(velocity));
                } else if char == '>' {
                    *index += 1;
                    return Some(MmlEvent::IncreOctave);
                } else if char == '<' {
                    *index += 1;
                    return Some(MmlEvent::DecreOctave);
                } else if char == ':' {
                    *index += 1;
                    return Some(MmlEvent::ConnectChord);
                } else {
                    let mml_note = get_first_mml_note(mml);
                    *index += mml_note.len();

                    let note = NoteEvent::from_mml(
                        mml_note,
                        current_mml_octave,
                        current_mml_velocity,
                        current_tempo,
                        *is_connect_chord,
                        *index,
                    );

                    *is_connect_chord = false;
                    return Some(MmlEvent::SetNote(note));
                }
            },
            None => None
        }
    }
}

fn get_first_mml_note(mml: &str) -> String {
    let mut chars = mml.chars();
    let mut result = String::new();
    let mut is_note_extra_checked = false;
    let mut before_char = chars.next().unwrap();
    let note_name = before_char;
    let to_match = ['&', '.', '+'];

    result.push(note_name);

    while let Some(char) = chars.next() {
        if is_note_extra_checked == false {
            if char == '+' {
                result.push(char);
                continue;
            }

            is_note_extra_checked = true;
        }

        let mut is_break = true;

        if char.is_digit(10) || to_match.contains(&char) {
            is_break = false;
        }

        if char == note_name && before_char == '&' {
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
    let mut chars = mml[1..].chars();
    let mut result = String::new();

    while let Some(char) = chars.next() {
        if char.is_digit(10) {
            result.push(char);
        } else {
            break;
        }
    }

    result
}
