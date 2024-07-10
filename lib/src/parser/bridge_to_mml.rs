use crate::{
    mml_event::{BridgeEvent, MmlEvent}, mml_note::MmlNote, mml_song::MmlSongOptions, Instrument
};

pub fn bridge_events_to_mml_events(
    bridge_events: Vec<BridgeEvent>,
    options: &MmlSongOptions,
    ppq: u16,
) -> (Vec<MmlEvent>, Instrument) {
    let mut mml_events: Vec<MmlEvent> = Vec::new();
    let mut before_note: Option<MmlNote> = None;
    let mut before_note_index: Option<usize> = None;
    let mut instrument = Instrument::default();

    for event in bridge_events {
        match event {
            BridgeEvent::Tempo(tempo, .. ) => {
                mml_events.push(MmlEvent::Tempo(tempo));
            }
            BridgeEvent::Note(midi_state) => {
                let note = MmlNote::from_midi_state(midi_state, options, ppq);
                
                if let Some(before_note) = &before_note {
                    // Rest and chord
                    let before_note_end = before_note.position_in_smallest_unit + before_note.duration_in_smallest_unit;
                    let position_diff = note.position_in_smallest_unit as isize - before_note_end as isize;

                    if position_diff > 0 {
                        mml_events.push(MmlEvent::Rest(position_diff as u8));
                    } else if position_diff < 0 {
                        if note.position_in_smallest_unit == before_note.position_in_smallest_unit {
                            mml_events.push(MmlEvent::ConnectChord);
                        } else {
                            // If current start is less than before end, cut before duration
                            if let Some(before_note_index) = before_note_index {
                                if let Some(before_note) = mml_events.get_mut(before_note_index) {
                                    if let MmlEvent::Note(before_note) = before_note {
                                        let new_duration = before_note.duration_in_smallest_unit as isize + position_diff;
                                        before_note.duration_in_smallest_unit = new_duration as usize;
                                    }
                                }
                            }
                        }
                    }

                    // Octave
                    let octave_diff = note.octave as i8 - before_note.octave as i8;

                    if octave_diff == 1 {
                        mml_events.push(MmlEvent::IncreOctave);
                    } else if octave_diff == -1 {
                        mml_events.push(MmlEvent::DecreOctave);
                    } else if octave_diff != 0 {
                        mml_events.push(MmlEvent::Octave(note.octave));
                    }

                    // Velocity
                    if note.velocity != before_note.velocity {
                        mml_events.push(MmlEvent::Velocity(note.velocity));
                    }
                }

                before_note = Some(note.to_owned());
                before_note_index = Some(mml_events.len());
                mml_events.push(MmlEvent::Note(note));
            }
            BridgeEvent::ProgramChange(dest_instrument, _) => {
                instrument = dest_instrument;
            }
        }
    }

    (mml_events, instrument)
}
