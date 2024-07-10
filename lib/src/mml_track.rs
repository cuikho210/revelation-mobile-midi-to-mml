use crate::{
    mml_event::{BridgeEvent, MmlEvent},
    mml_song::MmlSongOptions,
    parser::bridge_events_to_mml_events, Instrument,
};

#[derive(Debug, Clone)]
pub struct MmlTrack {
    pub instrument: Instrument,
    pub events: Vec<MmlEvent>
}

impl MmlTrack {
    pub fn from_bridge_events(
        bridge_events: Vec<BridgeEvent>,
        song_options: MmlSongOptions,
        ppq: u16,
    ) -> Self {
        let (events, instrument) = bridge_events_to_mml_events(
            bridge_events,
            &song_options,
            ppq,
        );

        Self { events, instrument }
    }

    pub fn to_mml(&self) -> String {
        let mut mml = String::new();

        for event in self.events.iter() {
            mml.push_str(&&event.to_mml_debug());
        }

        mml
    }
}
