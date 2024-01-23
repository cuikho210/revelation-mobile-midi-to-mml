use crate::utils;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum PitchClass {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
    Rest,
}

impl Display for PitchClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Self::C => "C",
            Self::Db => "C+",
            Self::D => "D",
            Self::Eb => "D+",
            Self::E => "E",
            Self::F => "F",
            Self::Gb => "F+",
            Self::G => "G",
            Self::Ab => "G+",
            Self::A => "A",
            Self::Bb => "A+",
            Self::B => "B",
            Self::Rest => "r",
        };

        write!(f, "{}", display_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Note {
    pub pitch_class: PitchClass,
    pub octave: u8,
    pub velocity: u8,

    pub duration_in_tick: u32,
    pub position_in_tick: u32,
    pub duration_in_smallest_unit: u32,
    pub position_in_smallest_unit: u32,
}

impl Note {
    pub fn new(ppq: u16, midi_key: u8, velocity: u8, current_tick: u32) -> Self {
        let pitch_class = utils::midi_key_to_pitch_class(midi_key);
        let octave = utils::midi_key_to_octave(midi_key);
        let position_in_smallest_unit = utils::tick_to_smallest_unit(current_tick, ppq);

        Self {
            pitch_class,
            octave,
            velocity,
            position_in_tick: current_tick,
            position_in_smallest_unit,
            duration_in_tick: 0,
            duration_in_smallest_unit: 0,
        }
    }

    pub fn to_mml(&self) -> String {
        utils::get_display_mml(self.duration_in_smallest_unit, &self.pitch_class)
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position_in_smallest_unit
            .cmp(&other.position_in_smallest_unit)
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
