use regex::Regex;
use crate::{
    mml_event::MmlEvent,
    note_event::NoteEvent,
};

const REGEXP_TEMPO: &str = r"t\d+";
const REGEXP_OCTAVE: &str = r"o\d+";
const REGEXP_VELOCITY: &str = r"v\d+";
const REGEXP_NOTE: &str = r"[ABCDEFGr](\+)?\d+(\.)?(&[ABCDEFGr](\+)?\d+(\.)?)*";

pub struct Parser {
    pub raw_mml: String,
}

impl Parser {
    pub fn parse(mml: String) -> Self {
        let result = Self {
            raw_mml: mml,
        };

        result.parse_note_events();

        result
    }

    fn parse_note_events(&self) {
        let mut index = 0usize;
        let mut current_mml_velocity = 12u8;
        let mut current_octave = 4u8;
        let mut current_tempo = 120u16;

        loop {
            if let Some(event) = self.parse_event(
                &mut index,
                current_mml_velocity,
                current_octave,
                current_tempo,
            ) {
                match event {
                    MmlEvent::SetNote(note) => println!("Note {} ==> {}", note.raw_mml, note.duration_in_note_64),
                    MmlEvent::SetTempo(tempo) => current_tempo = tempo,
                    MmlEvent::SetOctave(octave) => current_octave = octave,
                    MmlEvent::IncreOctave => current_octave += 1,
                    MmlEvent::DecreOctave => current_octave -= 1,
                    MmlEvent::SetVelocity(velocity) => current_mml_velocity = velocity,
                    MmlEvent::ConnectChord => (),
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
        current_tempo: u16,
    ) -> Option<MmlEvent> {
        match self.raw_mml.chars().nth(*index) {
            Some(char) => {
                let mml = &self.raw_mml.as_str()[*index..];

                if char == 't' {
                    let re = Regex::new(REGEXP_TEMPO).unwrap();
                    if let Some(matches) = re.find(mml) {
                        if matches.len() == 0 {
                            return None;
                        }

                        *index += matches.len();

                        let tempo = matches.as_str()[1..].parse::<u16>().unwrap();
                        return Some(MmlEvent::SetTempo(tempo));
                    } else {
                        return None;
                    }
                } else if char == 'o' {
                    let re = Regex::new(REGEXP_OCTAVE).unwrap();
                    if let Some(matches) = re.find(mml) {
                        if matches.len() == 0 {
                            return None;
                        }

                        *index += matches.len();
                        let octave = matches.as_str()[1..].parse::<u8>().unwrap();
                        return Some(MmlEvent::SetOctave(octave));
                    } else {
                        return None;
                    }
                } else if char == 'v' {
                    let re = Regex::new(REGEXP_VELOCITY).unwrap();
                    if let Some(matches) = re.find(mml) {
                        if matches.len() == 0 {
                            return None;
                        }

                        *index += matches.len();
                        let velocity = matches.as_str()[1..].parse::<u8>().unwrap();
                        return Some(MmlEvent::SetVelocity(velocity));
                    } else {
                        return None;
                    }
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
                    let re = Regex::new(REGEXP_NOTE).unwrap();
                    if let Some(matches) = re.find(mml) {
                        if matches.len() == 0 {
                            return None;
                        }

                        *index += matches.len();

                        let mml_note = matches.as_str();
                        let note = NoteEvent::from_mml(
                            mml_note.to_string(),
                            current_mml_octave,
                            current_mml_velocity,
                            current_tempo,
                        );

                        return Some(MmlEvent::SetNote(note));
                    } else {
                        return None;
                    }
                }
            },
            None => None
        }
    }
}
