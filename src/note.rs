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
    pub midi_channel: u8,
    pub midi_key: u8,
    pub midi_velocity: u8,
    pub pitch_class: PitchClass,
    pub octave: u8,
    pub velocity: u8,
    pub is_percussion: bool,

    pub duration_in_tick: u32,
    pub position_in_tick: u32,
    pub duration_in_smallest_unit: u32,
    pub position_in_smallest_unit: u32,
}

impl Note {
    pub fn new(
        ppq: u16,
        midi_channel: u8,
        midi_key: u8,
        midi_velocity: u8,
        velocity: u8,
        current_tick: u32,
    ) -> Self {
        let pitch_class = utils::midi_key_to_pitch_class(midi_key);
        let octave = utils::midi_key_to_octave(midi_key);
        let position_in_smallest_unit = utils::tick_to_smallest_unit(current_tick, ppq);
        let is_percussion = midi_channel == 10;

        let mut result = Self {
            midi_channel,
            midi_key,
            midi_velocity,
            pitch_class,
            octave,
            velocity,
            is_percussion,
            position_in_tick: current_tick,
            position_in_smallest_unit,
            duration_in_tick: 0,
            duration_in_smallest_unit: 0,
        };

        if is_percussion {
            result.to_percussion_note();
        }

        result
    }

    pub fn to_mml(&self) -> String {
        utils::get_display_mml(self.duration_in_smallest_unit, &self.pitch_class)
    }

    pub fn to_percussion_note(&mut self) {
        let new_midi_key = match self.midi_key {
            // Snare - C#4
            38|40 => 37,
            // Tom 1 - D4
            41 => 38,
            // Tom 2 - D#4
            43|50 => 39,
            // Tom 3 - E4
            45 => 40,
            // Hithat - F4
            42|44|46|54 => 41,
            // Crash 1 - F#4
            49 => 42,
            // Crash 2 - G4
            52 => 43,
            // Crash 3 - G#4
            55 => 44,
            // Crash 4 - A#4
            57 => 46,
            // Ride bell - A4
            51|53 => 45,
            // Cowbell - B4
            56 => 47,
            // Bongo - C5
            47|48 => 48,
            // Ride 2 - C#5
            58 => 49,
            // Stick - D5
            37 => 50,
            // Bush - D#5
            39|59 => 51,
            // Light bell - E5
            // => 52,
            // Kick - C4
            _ => 36,
        };

        self.midi_key = new_midi_key;
        self.pitch_class = utils::midi_key_to_pitch_class(new_midi_key);
        self.octave = utils::midi_key_to_octave(new_midi_key) + 2;
        self.midi_channel = 10;
        self.is_percussion = true;
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

