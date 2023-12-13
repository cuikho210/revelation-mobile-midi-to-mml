use std::{
    fs,
    path::Path,
    io::{Error, ErrorKind},
};
use midly::{
    Smf,
    Timing,
};
use crate::track::Track;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub ppq: u16,
    pub bpm: u16,
    pub tracks: Vec<Track>,
}

impl Song {
    pub fn from_path<P>(path: P) -> Result<Self, Error> where P: AsRef<Path> {
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(err) => return Err(err),
        };

        Self::from_bytes(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, Error> {
        let smf = match Smf::parse(&bytes) {
            Ok(smf) => smf,
            Err(err) => return Err(Error::new(ErrorKind::Other, err)),
        };

        let ppq = match Self::get_ppq_from_smf(&smf) {
            Some(ppq) => ppq,
            None => 480,
        };

        let mut bpm = 120u16;

        // Tracks
        let mut tracks: Vec<Track> = Vec::new();

        for smf_track in smf.tracks.iter() {
            tracks.push(Track::new(smf_track, ppq, &mut bpm));
        }

        Ok(Self {
            ppq, bpm,
            tracks,
        })
    }

    pub fn get_ppq_from_smf(smf: &Smf) -> Option<u16> {
        match smf.header.timing {
            Timing::Metrical(ppq) => Some(ppq.as_int()),
            _ => None,
        }
    }
}
