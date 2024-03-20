use crate::{track::Track, utils};
use midly::{Smf, Timing, Track as SmfTrack};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongOptions {
    pub auto_boot_velocity: bool,
    pub velocity_min: u8,
    pub velocity_max: u8,
}

impl Default for SongOptions {
    fn default() -> Self {
        SongOptions {
            auto_boot_velocity: true,
            velocity_min: 0,
            velocity_max: 15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub ppq: u16,
    pub bpm: u16,
    pub tracks: Vec<Track>,
    pub options: SongOptions,
}

impl Song {
    pub fn from_path<P>(path: P, options: SongOptions) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(err) => return Err(err),
        };

        Self::from_bytes(bytes, options)
    }

    pub fn from_bytes(bytes: Vec<u8>, options: SongOptions) -> Result<Self, Error> {
        let smf = match Smf::parse(&bytes) {
            Ok(smf) => smf,
            Err(err) => return Err(Error::new(ErrorKind::Other, err)),
        };

        let ppq = match get_ppq_from_smf(&smf) {
            Some(ppq) => ppq,
            None => 480,
        };

        let mut bpm = 120u16;

        let tracks = get_tracks(
            &smf.tracks,
            ppq,
            &mut bpm,
            &options
        );

        let mut result = Self { ppq, bpm, tracks, options };
        result.apply_song_options();

        Ok(result)
    }

    pub fn set_song_options(&mut self, options: SongOptions) {
        self.options = options;
        self.apply_song_options();
    }

    fn apply_song_options(&mut self) {
        self.apply_velocity_range(self.options.velocity_min, self.options.velocity_max);

        if self.options.auto_boot_velocity {
            auto_boot_note_velocity(&mut self.tracks, self.options.velocity_max);
        }
    }

    fn apply_velocity_range(&mut self, velocity_min: u8, velocity_max: u8) {
        for track in self.tracks.iter_mut() {
            track.apply_velocity_range(velocity_min, velocity_max);
        }
    }
}

fn get_tracks(
    smf_tracks: &Vec<SmfTrack>,
    ppq: u16,
    bpm: &mut u16,
    options: &SongOptions,
) -> Vec<Track> {
    let mut tracks: Vec<Track> = Vec::new();

    for smf_track in smf_tracks.iter() {
        let track = Track::new(
            smf_track,
            tracks.len().to_string(),
            ppq,
            bpm,
            options.velocity_min,
            options.velocity_max,
        );

        if track.notes.len() > 0 {
            tracks.push(track);
        }
    }

    tracks
}

fn auto_boot_note_velocity(tracks: &mut Vec<Track>, velocity_max: u8) {
    let mut max = 0u8;

    for track in tracks.iter() {
        let current = utils::get_highest_velocity(&track.notes);
        if current > max {
            max = current;
        }
    }

    let diff = velocity_max - max;

    for track in tracks.iter_mut() {
        track.apply_boot_velocity(diff);
    }
}

pub fn get_ppq_from_smf(smf: &Smf) -> Option<u16> {
    match smf.header.timing {
        Timing::Metrical(ppq) => Some(ppq.as_int()),
        _ => None,
    }
}
