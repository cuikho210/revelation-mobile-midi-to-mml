use crate::{mml_event::{BridgeEvent, MmlEvent}, mml_song::MmlSongOptions, parser::bridge_events_to_mml_events, Instrument};

#[derive(Debug, Clone)]
pub struct MmlTrack {
    pub name: String,
    pub instrument: Instrument,
    pub events: Vec<MmlEvent>,
    song_options: MmlSongOptions,
    bridge_events: Vec<BridgeEvent>,
    ppq: u16,
}

impl MmlTrack {
    pub fn from_bridge_events(
        index: usize,
        bridge_events: Vec<BridgeEvent>,
        song_options: MmlSongOptions,
        ppq: u16,
    ) -> Self {

        let mut mml_track = Self {
            name: index.to_string(),
            events: Vec::new(),
            instrument: Instrument::default(),
            bridge_events,
            song_options,
            ppq,
        };

        mml_track.generate_mml_events();
        mml_track
    }

    pub fn to_mml(&self) -> String {
        let mut mml = String::new();

        for event in self.events.iter() {
            mml.push_str(&event.to_mml(self.song_options.smallest_unit));
        }

        mml
    }

    pub fn to_mml_debug(&self) -> String {
        let mut mml = String::new();
        let mut notes_on_row: usize = 0;

        for event in self.events.iter() {
            let current_mml = &event.to_mml_debug(self.song_options.smallest_unit);
            mml.push_str(&current_mml);

            notes_on_row += event.get_duration();
            if notes_on_row >= 64 {
                notes_on_row = 0;
                mml.push('\n');
            }
        }

        mml
    }

    pub fn apply_boot_velocity(&mut self, velocity_diff: u8) {
        if velocity_diff > 0 {
            for event in self.events.iter_mut() {
                if let MmlEvent::Velocity(velocity) = event {
                    *velocity += velocity_diff
                }
            }
        }
    }

    fn generate_mml_events(&mut self) {
        let (events, instrument) = bridge_events_to_mml_events(
            &self.bridge_events,
            &self.song_options,
            self.ppq,
        );

        self.events = events;
        self.instrument = instrument;
    }
}
