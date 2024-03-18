use crate::{
    track::TrackEvent,
    note::Note,
    utils,
};

pub fn get_events_from_notes(notes: &Vec<Note>) -> Vec<TrackEvent> {
    let mut events: Vec<TrackEvent> = Vec::new();
    let mut max_end_position = 0u32;

    for index in 0..notes.len() {
        let note = notes.get(index).unwrap();
        let mut position_diff = note.position_in_smallest_unit;
        let mut octave_event: Option<TrackEvent> = None;

        if index > 0 {
            if let Some(before_note) = notes.get(index - 1) {
                let before_note_end_position =
                    before_note.position_in_smallest_unit + before_note.duration_in_smallest_unit;
                let mut is_connected_to_chord = false;

                // If while another note is playing
                if note.position_in_smallest_unit < before_note_end_position {
                    position_diff = 0;

                    is_connected_to_chord =
                        utils::try_connect_to_chord(&mut events, note, before_note);
                } else {
                    if position_diff >= max_end_position {
                        position_diff = position_diff - max_end_position;
                    } else {
                        position_diff = 0;
                    }
                }

                // Cut previous notes
                if !is_connected_to_chord && note.position_in_smallest_unit < max_end_position {
                    utils::cut_previous_notes(&mut events, note.position_in_smallest_unit);

                    max_end_position =
                        note.position_in_smallest_unit + note.duration_in_smallest_unit;
                }

                // Octave event
                let octave_diff = note.octave as i8 - before_note.octave as i8;

                if octave_diff == 1 {
                    octave_event = Some(TrackEvent::IncreOctave);
                } else if octave_diff == -1 {
                    octave_event = Some(TrackEvent::DecreOctave);
                } else if octave_diff != 0 {
                    octave_event = Some(TrackEvent::SetOctave(note.octave));
                }

                // Velocity
                if note.velocity != before_note.velocity {
                    events.push(TrackEvent::SetVelocity(note.velocity));
                }
            }
        } else {
            octave_event = Some(TrackEvent::SetOctave(note.octave));
            events.push(TrackEvent::SetVelocity(note.velocity));
        }

        if position_diff > 0 {
            events.push(TrackEvent::SetRest(position_diff));
        }

        if let Some(octave_event) = octave_event {
            events.push(octave_event);
        }

        let current_end_position = note.position_in_smallest_unit + note.duration_in_smallest_unit;
        if current_end_position > max_end_position {
            max_end_position = current_end_position;
        }

        events.push(TrackEvent::SetNote(note.to_owned()));
    }

    events
}
