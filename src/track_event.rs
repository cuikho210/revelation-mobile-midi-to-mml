use crate::note::Note;

#[derive(Debug, Clone)]
pub enum TrackEvent {
    /// Set the tempo with a bpm value.  
    /// Example: `t120`
    SetTempo(u32),

    /// Set the pitch octave.  
    /// Example: `o5`
    SetOctave(u8),

    /// Increment current octave by 1.  
    /// In MML: `>`
    IncreOctave,

    /// Decrement current octave by 1.  
    /// In MML: `<`
    DecreOctave,

    /// Changes the current note pitch without changing the duration.
    /// In MML: `&`
    ConnectNote,

    /// Connect current note with before note using `:` to create a chord
    ConnectChord,

    /// Set a note
    SetNote(Note),

    /// Example: `r4`
    SetRest(u8),
}

impl TrackEvent {
    pub fn to_mml(&self) -> String {
        return match self {
            Self::ConnectChord => String::from(":"),
            Self::ConnectNote => String::from("&"),
            Self::IncreOctave => String::from(">"),
            Self::DecreOctave => String::from("<"),
            Self::SetTempo(tempo) => format!("t{tempo}"),
            Self::SetOctave(octave) => format!("o{octave}"),
            Self::SetRest(rest) => format!("r{rest}"),
            Self::SetNote(note) => note.to_mml(),
        };
    }
}