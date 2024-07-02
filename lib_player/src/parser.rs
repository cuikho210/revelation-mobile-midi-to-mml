use regex::Regex;
use crate::{
    mml_event::MmlEvent,
    note_event::NoteEvent, SynthOutputConnection,
    utils,
};

const REGEXP_TEMPO: &str = r"t\d+";
const REGEXP_OCTAVE: &str = r"o\d+";
const REGEXP_VELOCITY: &str = r"v\d+";
const REGEXP_NOTE: &str = r"[ABCDEFGr](\+)?\d+(\.)?(&[ABCDEFGr](\+)?\d+(\.)?)*";

pub struct Parser {
    pub raw_mml: String,
    pub notes: Vec<NoteEvent>,
}

impl Parser {
    pub fn parse(mml: String) -> Self {
        let mut result = Self {
            raw_mml: mml,
            notes: Vec::new(),
        };

        result.parse_note_events();
        result
    }

    pub fn play(&self, connection: &mut SynthOutputConnection, channel: u8) {
        let mut before: Option<NoteEvent> = None;
        let mut current_chord: Vec<NoteEvent> = Vec::new();

        for note in self.notes.iter() {
            if note.is_connected_to_prev_note {
                if current_chord.len() == 0 {
                    current_chord.push(before.as_ref().unwrap().to_owned());
                }

                current_chord.push(note.to_owned());
                continue;
            }

            if current_chord.len() > 0 {
                utils::play_chord(connection, &current_chord, channel);
                current_chord.clear();
                continue;
            }

            if let Some(before_note) = &before {
                utils::play_note(connection, before_note, channel);
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
                    MmlEvent::SetNote(note) => self.notes.push(note),
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
                    let re = Regex::new(REGEXP_TEMPO).unwrap();
                    if let Some(matches) = re.find(mml) {
                        if matches.len() == 0 {
                            return None;
                        }

                        *index += matches.len();

                        let tempo = matches.as_str()[1..].parse::<usize>().unwrap();
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

                        let mml_note = matches.as_str();
                        let note = NoteEvent::from_mml(
                            mml_note.to_string(),
                            current_mml_octave,
                            current_mml_velocity,
                            current_tempo,
                            *is_connect_chord,
                        );

                        *index += matches.len();
                        *is_connect_chord = false;

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
