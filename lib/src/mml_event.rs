use crate::{
    mml_note::MmlNote,
    pitch_class::PitchClass,
    utils, Instrument,
};
use std::cmp::Ordering;

// --------------------------------
// Midi state
// --------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MidiState {
    pub position_in_tick: usize,
    pub duration_in_tick: usize,
    pub channel: u8,
}

impl Ord for MidiState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position_in_tick.cmp(&other.position_in_tick)
    }
}

impl PartialOrd for MidiState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MidiNoteState {
    pub key: u8,
    pub velocity: u8,
    pub midi_state: MidiState,
}

impl Ord for MidiNoteState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.midi_state.cmp(&other.midi_state)
    }
}

impl PartialOrd for MidiNoteState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// --------------------------------
// Bridge
// --------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeEvent {
    Note(MidiNoteState),
    Tempo(u32, MidiState),
    ProgramChange(Instrument, MidiState),
}

impl Ord for BridgeEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_position = match self {
            Self::Note(state) => state.midi_state.position_in_tick,
            Self::Tempo(_, pos) => pos.position_in_tick,
            Self::ProgramChange(_, pos) => pos.position_in_tick,
        };

        let other_position = match other {
            Self::Note(state) => state.midi_state.position_in_tick,
            Self::Tempo(_, pos) => pos.position_in_tick,
            Self::ProgramChange(_, pos) => pos.position_in_tick,
        };

        let order = self_position.cmp(&other_position);
        
        if let Ordering::Equal = order {
            match self {
                Self::Note(_) => {
                    match other {
                        Self::Note(_) => return Ordering::Equal,
                        _ => return Ordering::Greater,
                    }
                }
                _ => {
                    match other {
                        Self::Note(_) => return Ordering::Less,
                        _ => return Ordering::Equal,
                    }
                }
            }
        }

        order
    }
}

impl PartialOrd for BridgeEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// --------------------------------
// MML
// --------------------------------

#[derive(Debug, Clone)]
pub enum MmlEvent {
    Note(MmlNote),
    Rest(usize),
    Tempo(u32),
    Octave(u8),
    IncreOctave,
    DecreOctave,
    ConnectChord,
    Velocity(u8),
    NoteLength(u8),
}

impl MmlEvent {
    pub fn to_mml(&self, smallest_unit: usize) -> String {
        match self {
            Self::ConnectChord => String::from(":"),
            Self::IncreOctave => String::from(">"),
            Self::DecreOctave => String::from("<"),
            Self::Tempo(tempo) => format!("t{tempo}"),
            Self::Octave(octave) => format!("o{octave}"),
            Self::Note(note) => utils::get_display_mml(note.duration_in_smallest_unit, &note.pitch_class, smallest_unit),
            Self::Rest(rest) => utils::get_display_mml(rest.to_owned().into(), &PitchClass::Rest, smallest_unit),
            Self::Velocity(vel) => format!("v{}", vel),
            Self::NoteLength(length) => format!("l{}", length),
        }
    }

    pub fn to_mml_debug(&self, smallest_unit: usize) -> String {
        match self {
            Self::ConnectChord => String::from(":"),
            Self::IncreOctave => String::from(">"),
            Self::DecreOctave => String::from("<"),
            Self::Tempo(tempo) => format!("\nt{tempo}\n"),
            Self::Octave(octave) => format!(" o{octave} "),
            Self::Note(note) => {
                format!(
                    "{} ",
                    utils::get_display_mml(note.duration_in_smallest_unit, &note.pitch_class, smallest_unit)
                )
            }
            Self::Rest(rest) => {
                format!(
                    "{} ",
                    utils::get_display_mml(rest.to_owned().into(), &PitchClass::Rest, smallest_unit)
                )
            },
            Self::Velocity(vel) => format!(" v{} ", vel),
            Self::NoteLength(length) => format!(" l{} ", length),
        }
    }

    /// Get duration in smallest unit
    pub fn get_duration(&self) -> usize {
        match self {
            Self::Note(note) => note.duration_in_smallest_unit,
            Self::Rest(rest) => rest.to_owned(),
            _ => 0
        }
    }
}
