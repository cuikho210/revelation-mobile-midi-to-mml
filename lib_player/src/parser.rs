use revelation_mobile_midi_to_mml::{Note, TrackEvent};
use regex::Regex;

const REGEXP_TEMPO: &str = r"t\d+";
const REGEXP_OCTAVE: &str = r"o\d+";
const REGEXP_VELOCITY: &str = r"v\d+";
const REGEXP_NOTE: &str = r"[ABCDEFGr](\+)?\d+(\.)?(&[ABCDEFGr](\+)?\d+(\.)?)*";

const DEFAULT_PPQ: u16 = 480;

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
                    TrackEvent::SetNote(note) => println!("Note {} - {}", note.pitch_class, note.midi_key),
                    TrackEvent::SetRest(_rest) => (),
                    TrackEvent::SetTempo(tempo) => current_tempo = tempo,
                    TrackEvent::SetOctave(octave) => current_octave = octave,
                    TrackEvent::IncreOctave => current_octave += 1,
                    TrackEvent::DecreOctave => current_octave -= 1,
                    TrackEvent::SetVelocity(velocity) => current_mml_velocity = velocity,
                    TrackEvent::ConnectChord => (),
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
        _current_tempo: u16,
    ) -> Option<TrackEvent> {
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
                        return Some(TrackEvent::SetTempo(tempo));
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
                        return Some(TrackEvent::SetOctave(octave));
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
                        return Some(TrackEvent::SetVelocity(velocity));
                    } else {
                        return None;
                    }
                } else if char == '>' {
                    *index += 1;
                    return Some(TrackEvent::IncreOctave);
                } else if char == '<' {
                    *index += 1;
                    return Some(TrackEvent::DecreOctave);
                } else if char == ':' {
                    *index += 1;
                    return Some(TrackEvent::ConnectChord);
                } else {
                    let re = Regex::new(REGEXP_NOTE).unwrap();
                    if let Some(matches) = re.find(mml) {
                        if matches.len() == 0 {
                            return None;
                        }

                        *index += matches.len();

                        let mml_note = matches.as_str();
                        let midi_key = mml_to_midi_key(mml_note, current_mml_octave);
                        let midi_velocity = mml_velocity_to_midi_velocity(current_mml_velocity);
                        let duration_in_note_64 = 64;

                        if let Some(key) = midi_key {
                            let note = Note::new(
                                DEFAULT_PPQ,
                                0,
                                key,
                                midi_velocity,
                                current_mml_velocity,
                                0
                            );

                            return Some(TrackEvent::SetNote(note));
                        } else {
                            return Some(TrackEvent::SetRest(duration_in_note_64));
                        }
                    } else {
                        return None;
                    }
                }
            },
            None => None
        }
    }
}

pub fn mml_velocity_to_midi_velocity(mml_velocity: u8) -> u8 {
    mml_velocity / 15 * 128
}

pub fn mml_to_midi_key(mml: &str, octave: u8) -> Option<u8> {
    let mml = mml.to_lowercase();
    let char_0 = mml.chars().next().unwrap();
    let char_1 = mml.chars().next().unwrap();

    let key: Option<u8> = match char_0 {
        'c' => Some(12),
        'd' => Some(14),
        'e' => Some(16),
        'f' => Some(17),
        'g' => Some(19),
        'a' => Some(21),
        'b' => Some(23),
        _ => None,
    };

    if let Some(key_value) = key {
        let mut result = key_value;
        result += octave * 12;

        if char_1 == '+' {
            result += 1;
        }

        Some(result)
    } else {
        None
    }
}
