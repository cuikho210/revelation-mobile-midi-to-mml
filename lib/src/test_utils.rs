use std::fs;

use midly::{Smf, Timing};

use crate::{
    BridgeEvent, MmlSongOptions,
    parser::{bridge_meta_from_midi_track, bridge_notes_from_midi_track},
};

pub const MIDI_PATHS: [&str; 3] = [
    "../assets/heart_beat-band.mid",
    "../assets/FIRE_BIRD_(full_ver_)_(BanG_Dream!_Roselia_9th_Single)_(piano_cover).mid",
    "../assets/Stay_With_Me_-_Miki_Matsubara.mid",
];

pub fn setup_bridge_events(midi_path: &str) -> (Vec<BridgeEvent>, MmlSongOptions, u16) {
    let bytes = fs::read(midi_path).unwrap();
    let smf = Smf::parse(&bytes).unwrap();
    let smf_track = smf.tracks.first().unwrap();
    let bridge_events = {
        let meta_bridge_events = bridge_meta_from_midi_track(smf_track);
        let mut note_bridge_events = bridge_notes_from_midi_track(smf_track);
        note_bridge_events.extend(meta_bridge_events);
        note_bridge_events.sort();
        note_bridge_events
    };
    let options = MmlSongOptions::default();
    let ppq = match smf.header.timing {
        Timing::Metrical(ppq) => ppq.as_int(),
        _ => 480,
    };

    (bridge_events, options, ppq)
}
