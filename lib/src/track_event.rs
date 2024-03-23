use crate::{
    note::{Note, PitchClass},
    utils,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrackEvent {
    /// Set the tempo with a bpm value.  
    /// Example: `t120`
    SetTempo(u16),

    /// Set the pitch octave.  
    /// Example: `o5`
    SetOctave(u8),

    /// Increment current octave by 1.  
    /// In MML: `>`
    IncreOctave,

    /// Decrement current octave by 1.  
    /// In MML: `<`
    DecreOctave,

    /// Connect current note with before note using `:` to create a chord
    ConnectChord,

    /// Set a note
    SetNote(Note),

    /// Store a value is the duration in note 64
    SetRest(u32),

    SetVelocity(u8),
}

impl TrackEvent {
    pub fn to_mml(&self) -> String {
        match self {
            Self::ConnectChord => String::from(":"),
            Self::IncreOctave => String::from(">"),
            Self::DecreOctave => String::from("<"),
            Self::SetTempo(tempo) => format!("t{tempo}"),
            Self::SetOctave(octave) => format!("o{octave}"),
            Self::SetNote(note) => note.to_mml(),
            Self::SetRest(rest) => utils::get_display_mml(rest.to_owned(), &PitchClass::Rest),
            Self::SetVelocity(vel) => format!("v{}", vel),
        }
    }

    pub fn update_mml_note(&mut self) {
        match self {
            Self::SetNote(note) => note.update_mml_string(),
            _ => ()
        }
    }

    pub fn count_mml_note(&self) -> usize {
        match self {
            Self::SetNote(note) => note.count_mml_note(),
            Self::SetRest(_) => self.to_mml().split("&").count(),
            _ => 0
        }
    }
}
