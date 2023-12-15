use crate::{
    note::Note,
    utils,
};
use serde::{Serialize, Deserialize};

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
}

impl TrackEvent {
    pub fn to_mml(&self) -> String {
        return match self {
            Self::ConnectChord => String::from(":"),
            Self::IncreOctave => String::from(">"),
            Self::DecreOctave => String::from("<"),
            Self::SetTempo(tempo) => format!("t{tempo}"),
            Self::SetOctave(octave) => format!("o{octave}"),
            Self::SetNote(note) => note.to_mml(),
            Self::SetRest(rest) => utils::get_display_mml(rest.to_owned(), &"r".to_string()),
        };
    }
}
