use std::{fmt::Debug, fs, io::{Error, ErrorKind}, path::Path, thread::{self, JoinHandle}};
use midly::{Smf, Timing, TrackEvent};
use crate::{mml_event::BridgeEvent, parser::{bridge_notes_from_midi_track, bridge_meta_from_midi_track}, mml_track::MmlTrack};

#[derive(Debug, Clone)]
pub struct MmlSongOptions {
    pub auto_boot_velocity: bool,
    pub velocity_min: u8,
    pub velocity_max: u8,
}

impl Default for MmlSongOptions {
    fn default() -> Self {
        Self {
            auto_boot_velocity: false,
            velocity_min: 0,
            velocity_max: 15,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MmlSong {
    pub ppq: u16,
    pub tracks: Vec<MmlTrack>,
}

impl MmlSong {
    pub fn from_path<P>(path: P, options: MmlSongOptions) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(err) => return Err(err),
        };

        Self::from_bytes(bytes, options)
    }

    pub fn from_bytes(bytes: Vec<u8>, options: MmlSongOptions) -> Result<Self, Error> {
        let smf = match Smf::parse(&bytes) {
            Ok(smf) => smf,
            Err(err) => return Err(Error::new(ErrorKind::Other, err)),
        };

        let ppq = match get_ppq_from_smf(&smf) {
            Some(ppq) => ppq,
            None => 480,
        };

        let meta_events = get_bridge_meta_events(&smf.tracks);

        let smf_tracks = smf.make_static().tracks;
        let mut bridge_note_events = get_bridge_note_events(smf_tracks);
        apply_meta_events(&mut bridge_note_events, &meta_events);

        let tracks = bridge_events_to_tracks(bridge_note_events, &options, ppq);

        let result = Self { ppq, tracks };
        Ok(result)
    }
}

fn bridge_events_to_tracks(
    bridge_events: Vec<Vec<BridgeEvent>>,
    song_options: &MmlSongOptions,
    ppq: u16,
) -> Vec<MmlTrack> {
    let mut tracks: Vec<MmlTrack> = Vec::new();
    let mut handles: Vec<JoinHandle<MmlTrack>> = Vec::new();

    for events in bridge_events {
        let options = song_options.to_owned();
        let handle = thread::spawn::<_, MmlTrack>(move || {
            MmlTrack::from_bridge_events(events, options, ppq)
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Ok(track) => {
                tracks.push(track);
            }
            Err(_) => {
                eprintln!("[bridge_events_to_tracks] Cannot join thread");
            }
        }
    }

    tracks
}

fn apply_meta_events(list_note_events: &mut Vec<Vec<BridgeEvent>>, meta_events: &Vec<BridgeEvent>) {
    for meta_event in meta_events.iter() {
        for note_events in list_note_events.iter_mut() {
            note_events.push(meta_event.to_owned());
        }
    }

    for note_events in list_note_events.iter_mut() {
        note_events.sort();
    }
}

fn get_bridge_note_events(smf_tracks: Vec<Vec<TrackEvent<'static>>>) -> Vec<Vec<BridgeEvent>> {
    let mut events: Vec<Vec<BridgeEvent>> = Vec::new();
    let mut handles: Vec<JoinHandle<Vec<BridgeEvent>>> = Vec::new();

    for track in smf_tracks {
        let handle = thread::spawn::<_, Vec<BridgeEvent>>(move || {
            bridge_notes_from_midi_track(&track)
        });
        handles.push(handle);
    }
    
    for handle in handles {
        match handle.join() {
            Ok(note_events) => {
                events.push(note_events);
            }
            Err(_) => {
                eprintln!("[bridge_smf_tracks] Cannot join thread");
            }
        }
    }

    events
}

fn get_bridge_meta_events(smf_tracks: &Vec<Vec<TrackEvent>>) -> Vec<BridgeEvent> {
    let mut meta_events: Vec<BridgeEvent> = Vec::new();

    for track in smf_tracks.iter() {
        let mut events = bridge_meta_from_midi_track(track);
        meta_events.append(&mut events);
    }

    meta_events
}

fn get_ppq_from_smf(smf: &Smf) -> Option<u16> {
    match smf.header.timing {
        Timing::Metrical(ppq) => Some(ppq.as_int()),
        _ => None,
    }
}
