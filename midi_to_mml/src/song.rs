use crate::track::Track;
use midly::{Smf, Timing};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub ppq: u16,
    pub bpm: u16,
    pub tracks: Vec<Track>,
}

impl Song {
    pub fn from_path<P>(path: P, split_track: bool) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(err) => return Err(err),
        };

        Self::from_bytes(bytes, split_track)
    }

    pub fn from_bytes(bytes: Vec<u8>, split_track: bool) -> Result<Self, Error> {
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
            let track = Track::new(smf_track, ppq, &mut bpm);

            if track.notes.len() > 0 {
                tracks.push(track);
            }
        }

        // Split track
        if split_track {
            if tracks.len() == 1 {
                let (a, b) = tracks.first().unwrap().split();
                tracks = vec![a, b];
            } else if tracks.len() == 2 {
                let track_a = tracks.get(0).unwrap();
                let track_b = tracks.get(1).unwrap();

                if track_a.notes.len() > track_b.notes.len() {
                    let (a, b) = track_a.split();
                    tracks = vec![a, b, track_b.to_owned()];
                } else {
                    let (a, b) = track_b.split();
                    tracks = vec![track_a.to_owned(), a, b].to_owned();
                }
            }
        }

        Ok(Self { ppq, bpm, tracks })
    }

    pub fn get_ppq_from_smf(smf: &Smf) -> Option<u16> {
        match smf.header.timing {
            Timing::Metrical(ppq) => Some(ppq.as_int()),
            _ => None,
        }
    }
}
