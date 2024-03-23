mod notes;
mod events;
mod fixer;
mod track_utils;

use crate::{
    Note,
    TrackEvent,
    Instrument,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub name: String,
    pub events: Vec<TrackEvent>,
    pub notes: Vec<Note>,
    pub ppq: u16,
    pub bpm: u16,
    pub instrument: Instrument,
    pub mml_note_length: usize,
}

impl Track {
    pub fn new(
        smf_track: &midly::Track,
        name: String,
        ppq: u16, bpm: &mut u16,
        velocity_min: u8,
        velocity_max: u8,
    ) -> Self {
        if let Some(new_bpm) = track_utils::get_bpm_from_smf_track(smf_track) {
            *bpm = new_bpm;
        };

        let instrument = track_utils::get_instrument_from_track(smf_track);
        let notes = notes::get_notes_from_smf_track(smf_track, ppq, velocity_min, velocity_max);

        let mut result = Self {
            name,
            events: Vec::new(),
            notes,
            ppq,
            bpm: *bpm,
            instrument,
            mml_note_length: 0,
        };

        result.update_events();
        result.update_mml_note_length();
        result
    }

    pub fn from_notes(name: String, ppq: u16, bpm: u16, instrument: Instrument, notes: Vec<Note>) -> Self {
        let mut result = Self {
            name,
            events: Vec::new(),
            notes,
            ppq,
            bpm,
            instrument,
            mml_note_length: 0,
        };

        result.update_events();
        result.update_mml_note_length();
        result
    }

    pub fn to_mml(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        result.push(TrackEvent::SetTempo(self.bpm).to_mml());

        for event in self.events.iter() {
            result.push(event.to_mml());
        }

        result.join("")
    }

    /// Merge other track to this track
    pub fn merge(&mut self, other: &Self) {
        self.notes.append(&mut other.notes.to_owned());

        // Sort by position_in_smallest_unit
        self.notes.sort();
        self.update_events();
        self.update_mml_note_length();
    }

    pub fn split(&self) -> (Self, Self) {
        track_utils::split_track(self)
    }

    pub fn apply_boot_velocity(&mut self, diff: u8) {
        fixer::apply_boot_velocity(&mut self.notes, diff);
        self.update_events();
    }

    pub fn apply_velocity_range(&mut self, velocity_min: u8, velocity_max: u8) {
        fixer::apply_velocity_range(&mut self.notes, velocity_min, velocity_max);
        self.update_events();
    }

    pub fn to_percussion(&mut self) {
        for note in self.notes.iter_mut() {
            note.to_percussion_note();
        }

        self.instrument = Instrument::new(0, 10);
        self.update_events();
    }

    fn update_events(&mut self) {
        self.events = events::get_events_from_notes(&self.notes);
        fixer::fix_chord_duration(&mut self.events);
        fixer::fix_note_position(&mut self.events);
    }

    fn update_mml_note_length(&mut self) {
        let mut note_length = 0usize;
        
        for event in self.events.iter() {
            note_length += event.count_mml_note();
        }

        self.mml_note_length = note_length;
    }
}
