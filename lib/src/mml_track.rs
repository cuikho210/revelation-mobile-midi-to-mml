use crate::{mml_event::{BridgeEvent, MmlEvent}, mml_song::MmlSongOptions, parser::bridge_events_to_mml_events, Instrument};

#[derive(Debug, Clone)]
pub struct MmlTrack {
    pub instrument: Instrument,
    pub events: Vec<MmlEvent>,
    pub song_options: MmlSongOptions,
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

        Self { events, instrument, song_options }
    }

    pub fn to_mml(&self) -> String {
        let mut mml = String::new();

        for event in self.events.iter() {
            mml.push_str(&event.to_mml(self.song_options.smallest_unit as usize));
        }

        mml
    }

    pub fn to_mml_debug(&self) -> String {
        let mut mml = String::new();
        let mut notes_on_row: usize = 0;

        for event in self.events.iter() {
            let current_mml = &event.to_mml_debug(self.song_options.smallest_unit as usize);
            mml.push_str(&current_mml);

            notes_on_row += event.get_duration();
            if notes_on_row >= 64 {
                notes_on_row = 0;
                mml.push('\n');
            }
        }

        mml
    }
}
