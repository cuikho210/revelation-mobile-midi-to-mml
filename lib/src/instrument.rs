use crate::instrument_map::INSTRUMENT_MAP;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instrument {
    pub name: String,
    pub instrument_id: u8,
    pub midi_channel: u8,
}

impl Instrument {
    pub fn new(instrument_id: u8, midi_channel: u8) -> Self {
        Self {
            instrument_id,
            midi_channel,
            name: match_instrument_name(instrument_id, midi_channel),
        }
    }
}

impl Default for Instrument {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

fn match_instrument_name(instrument_id: u8, midi_channel: u8) -> String {
    if midi_channel == 9 {
        return "Drum Set".to_string();
    }

    match INSTRUMENT_MAP.get(instrument_id as usize) {
        Some(str) => str.to_string(),
        None => INSTRUMENT_MAP.first().unwrap().to_string(),
    }
}
