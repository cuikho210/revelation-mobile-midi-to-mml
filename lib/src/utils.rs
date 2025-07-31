use crate::{MmlSongOptions, mml_event::MmlEvent, mml_track::MmlTrack, pitch_class::PitchClass};
use rayon::prelude::*;
use std::convert::TryInto;

pub fn count_mml_notes(mml_string: &str) -> usize {
    mml_string.split("&").count()
}

pub fn equalize_tracks(track_a: &mut MmlTrack, track_b: &mut MmlTrack) {
    let equalize = |a: &mut MmlTrack, b: &mut MmlTrack, gap: usize| {
        let mut mml_counter = 0usize;
        let mut index_counter = 0usize;

        for (index, event) in a.events.iter().enumerate() {
            if let MmlEvent::Note(note) = event {
                mml_counter += note.mml_note_length;

                if mml_counter >= gap {
                    index_counter = index;
                    break;
                }
            }
        }

        let ratio = index_counter as f32 / a.events.len() as f32;
        let bridge_event_center_index = a.bridge_note_events.len() as f32 * ratio;

        let (left, right) = a
            .bridge_note_events
            .split_at(bridge_event_center_index.floor() as usize);
        let mut left = left.to_vec();
        let right = right.to_vec();

        a.bridge_note_events = right;
        a.generate_mml_events();

        b.bridge_note_events.append(&mut left);
        b.generate_mml_events();
    };

    let length_a = track_a.mml_note_length as isize;
    let length_b = track_b.mml_note_length as isize;
    let gap = (length_a - length_b) / 2;

    if gap > 0 {
        equalize(track_a, track_b, gap as usize);
    } else {
        equalize(track_b, track_a, gap.unsigned_abs());
    }
}

pub fn get_song_velocity_diff(song_options: &MmlSongOptions, tracks: &[MmlTrack]) -> u8 {
    let velocity_max: u8 = tracks
        .par_iter()
        .map(|track| get_highest_velocity(&track.events))
        .max()
        .unwrap_or(0);

    song_options.velocity_max - velocity_max
}

pub fn auto_boot_song_velocity(tracks: &mut [MmlTrack], velocity_diff: u8) {
    tracks
        .par_iter_mut()
        .for_each(|track| track.apply_boot_velocity(velocity_diff));
}

pub fn midi_velocity_to_mml_velocity(midi_velocity: u8, velocity_min: u8, velocity_max: u8) -> u8 {
    // Handle invalid range by swapping min and max
    let (actual_min, actual_max) = if velocity_min > velocity_max {
        (velocity_max, velocity_min)
    } else {
        (velocity_min, velocity_max)
    };

    let range: i32 = (actual_max - actual_min).into();
    let midi_velocity: i32 = midi_velocity.into();
    let velocity_min: i32 = actual_min.into();

    ((midi_velocity * range / 127) + velocity_min)
        .try_into()
        .unwrap()
}

pub fn get_highest_velocity(events: &[MmlEvent]) -> u8 {
    let mut max = 0u8;

    for event in events.iter() {
        if let MmlEvent::Velocity(vel) = event
            && *vel > max
        {
            max = *vel;
        }
    }

    max
}

pub fn midi_key_to_pitch_class(midi_key: u8) -> PitchClass {
    let classes: [PitchClass; 12] = [
        PitchClass::C,
        PitchClass::Db,
        PitchClass::D,
        PitchClass::Eb,
        PitchClass::E,
        PitchClass::F,
        PitchClass::Gb,
        PitchClass::G,
        PitchClass::Ab,
        PitchClass::A,
        PitchClass::Bb,
        PitchClass::B,
    ];
    let index = midi_key % 12;
    classes[index as usize].to_owned()
}

pub fn midi_key_to_octave(midi_key: u8) -> u8 {
    if midi_key < 12 {
        // Handle octave -1 case by returning 0 (could also use i8 for proper negative octaves)
        0
    } else {
        (midi_key / 12) - 1
    }
}

pub fn get_smallest_unit_in_tick(ppq: u16, smallest_unit: usize) -> f32 {
    ppq as f32 / (smallest_unit as f32 / 4.)
}

pub fn tick_to_smallest_unit(tick: usize, ppq: u16, smallest_unit: usize) -> usize {
    let note = get_smallest_unit_in_tick(ppq, smallest_unit);
    let duration_in_note = tick as f32 / note;

    duration_in_note.round() as usize
}

#[derive(Debug, Clone, PartialEq)]
struct CustomMmlNote {
    duration_in_smallest_unit: usize,
    mml_value: usize,
}

impl CustomMmlNote {
    pub fn new(smallest_unit: usize, duration_in_smallest_unit: usize) -> Self {
        Self {
            duration_in_smallest_unit,
            mml_value: smallest_unit / duration_in_smallest_unit,
        }
    }
}

fn get_list_of_mml_notes(smallest_unit: usize) -> Vec<CustomMmlNote> {
    let mut notes: Vec<CustomMmlNote> = Vec::new();
    let mut remainder = smallest_unit;

    while remainder > 1 {
        notes.push(CustomMmlNote::new(smallest_unit, remainder));
        remainder /= 2;
    }
    notes.push(CustomMmlNote::new(smallest_unit, remainder));

    notes
}

pub fn get_display_mml(
    mut duration_in_smallest_unit: usize,
    note_class: &PitchClass,
    smallest_unit: usize,
) -> String {
    let mut result: Vec<String> = Vec::new();
    let notes = get_list_of_mml_notes(smallest_unit);

    while duration_in_smallest_unit > 0 {
        let mut current_note: usize = 0;

        for mml_note in notes.iter() {
            if duration_in_smallest_unit >= mml_note.duration_in_smallest_unit {
                duration_in_smallest_unit -= mml_note.duration_in_smallest_unit;
                current_note = mml_note.mml_value;
                break;
            }
        }

        result.push(format!("{}{}", note_class, current_note));

        let half_of_current_note = smallest_unit / (current_note * 2);
        if duration_in_smallest_unit > 0 && duration_in_smallest_unit >= half_of_current_note {
            result.push(".".to_string());
            duration_in_smallest_unit -= half_of_current_note;
        }

        if duration_in_smallest_unit == 0 {
            break;
        } else {
            result.push("&".to_string());
        }
    }

    result.join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MmlEvent, PitchClass};

    #[test]
    fn test_count_mml_notes() {
        assert_eq!(count_mml_notes("c4"), 1);
        assert_eq!(count_mml_notes("c4&c8"), 2);
        assert_eq!(count_mml_notes("c4&c8&c16"), 3);
        assert_eq!(count_mml_notes(""), 1); // empty string has 1 part when split
    }

    #[test]
    fn test_midi_velocity_to_mml_velocity() {
        // Test full range mapping
        assert_eq!(midi_velocity_to_mml_velocity(0, 0, 15), 0);
        assert_eq!(midi_velocity_to_mml_velocity(127, 0, 15), 15);
        assert_eq!(midi_velocity_to_mml_velocity(63, 0, 15), 7); // roughly middle

        // Test custom range
        assert_eq!(midi_velocity_to_mml_velocity(0, 5, 10), 5);
        assert_eq!(midi_velocity_to_mml_velocity(127, 5, 10), 10);

        // Test edge cases
        assert_eq!(midi_velocity_to_mml_velocity(1, 0, 15), 0);
        assert_eq!(midi_velocity_to_mml_velocity(126, 0, 15), 14);
    }

    #[test]
    fn test_midi_key_to_pitch_class() {
        // Test C notes
        assert_eq!(midi_key_to_pitch_class(60), PitchClass::C); // Middle C
        assert_eq!(midi_key_to_pitch_class(48), PitchClass::C); // C3
        assert_eq!(midi_key_to_pitch_class(72), PitchClass::C); // C5

        // Test chromatic scale from C
        assert_eq!(midi_key_to_pitch_class(60), PitchClass::C);
        assert_eq!(midi_key_to_pitch_class(61), PitchClass::Db);
        assert_eq!(midi_key_to_pitch_class(62), PitchClass::D);
        assert_eq!(midi_key_to_pitch_class(63), PitchClass::Eb);
        assert_eq!(midi_key_to_pitch_class(64), PitchClass::E);
        assert_eq!(midi_key_to_pitch_class(65), PitchClass::F);
        assert_eq!(midi_key_to_pitch_class(66), PitchClass::Gb);
        assert_eq!(midi_key_to_pitch_class(67), PitchClass::G);
        assert_eq!(midi_key_to_pitch_class(68), PitchClass::Ab);
        assert_eq!(midi_key_to_pitch_class(69), PitchClass::A);
        assert_eq!(midi_key_to_pitch_class(70), PitchClass::Bb);
        assert_eq!(midi_key_to_pitch_class(71), PitchClass::B);
    }

    #[test]
    fn test_midi_key_to_octave() {
        // Test standard octave mapping
        assert_eq!(midi_key_to_octave(60), 4); // Middle C is C4
        assert_eq!(midi_key_to_octave(48), 3); // C3
        assert_eq!(midi_key_to_octave(72), 5); // C5

        // Test edge cases - now fixed to handle underflow
        assert_eq!(midi_key_to_octave(0), 0); // Fixed: no longer underflows
        assert_eq!(midi_key_to_octave(12), 0);
        assert_eq!(midi_key_to_octave(127), 9);
    }

    #[test]
    fn test_get_smallest_unit_in_tick() {
        // Standard 4/4 time with quarter note = 480 ticks
        let ppq = 480;

        // Test different smallest units
        assert_eq!(get_smallest_unit_in_tick(ppq, 64), 30.0); // 1/64 note = 30 ticks
        assert_eq!(get_smallest_unit_in_tick(ppq, 32), 60.0); // 1/32 note = 60 ticks
        assert_eq!(get_smallest_unit_in_tick(ppq, 16), 120.0); // 1/16 note = 120 ticks
        assert_eq!(get_smallest_unit_in_tick(ppq, 4), 480.0); // 1/4 note = 480 ticks
    }

    #[test]
    fn test_tick_to_smallest_unit() {
        let ppq = 480;
        let smallest_unit = 64;

        // Test exact conversions
        assert_eq!(tick_to_smallest_unit(480, ppq, smallest_unit), 16); // Quarter note = 16 * 1/64
        assert_eq!(tick_to_smallest_unit(240, ppq, smallest_unit), 8); // Eighth note = 8 * 1/64
        assert_eq!(tick_to_smallest_unit(120, ppq, smallest_unit), 4); // Sixteenth note = 4 * 1/64
        assert_eq!(tick_to_smallest_unit(30, ppq, smallest_unit), 1); // 1/64 note = 1 * 1/64

        // Test rounding
        assert_eq!(tick_to_smallest_unit(29, ppq, smallest_unit), 1); // Rounds to nearest
        assert_eq!(tick_to_smallest_unit(31, ppq, smallest_unit), 1); // Rounds to nearest
        assert_eq!(tick_to_smallest_unit(45, ppq, smallest_unit), 2); // Rounds up
    }

    #[test]
    fn test_get_display_mml_basic_notes() {
        let smallest_unit = 64;

        // Test basic note durations
        assert_eq!(get_display_mml(16, &PitchClass::C, smallest_unit), "c4"); // Quarter note
        assert_eq!(get_display_mml(8, &PitchClass::D, smallest_unit), "d8"); // Eighth note
        assert_eq!(get_display_mml(4, &PitchClass::E, smallest_unit), "e16"); // Sixteenth note
        assert_eq!(get_display_mml(1, &PitchClass::F, smallest_unit), "f64"); // 1/64 note

        // Test rests
        assert_eq!(get_display_mml(16, &PitchClass::Rest, smallest_unit), "r4");
        assert_eq!(get_display_mml(8, &PitchClass::Rest, smallest_unit), "r8");
    }

    #[test]
    fn test_get_display_mml_dotted_notes() {
        let smallest_unit = 64;

        // Test dotted notes (note + half duration)
        assert_eq!(get_display_mml(24, &PitchClass::C, smallest_unit), "c4."); // Dotted quarter
        assert_eq!(get_display_mml(12, &PitchClass::D, smallest_unit), "d8."); // Dotted eighth
        assert_eq!(get_display_mml(6, &PitchClass::E, smallest_unit), "e16."); // Dotted sixteenth
    }

    #[test]
    fn test_get_display_mml_tied_notes() {
        let smallest_unit = 64;

        // Test tied notes based on actual behavior
        assert_eq!(get_display_mml(48, &PitchClass::C, smallest_unit), "c2."); // Half note + dot (32+16=48)
        assert_eq!(get_display_mml(20, &PitchClass::D, smallest_unit), "d4&d16"); // Quarter + sixteenth (16+4=20)
        assert_eq!(get_display_mml(56, &PitchClass::E, smallest_unit), "e2.&e8"); // Dotted half + eighth (48+8=56)
    }

    #[test]
    fn test_get_display_mml_edge_cases() {
        let smallest_unit = 64;

        // Test zero duration - should not hang
        assert_eq!(get_display_mml(0, &PitchClass::C, smallest_unit), "");

        // Test very long duration - based on actual behavior
        assert_eq!(
            get_display_mml(128, &PitchClass::C, smallest_unit),
            "c1.&c2"
        ); // Dotted whole + half (96+32=128)
    }

    #[test]
    fn test_get_highest_velocity() {
        let events = vec![
            MmlEvent::Velocity(5),
            MmlEvent::Velocity(10),
            MmlEvent::Velocity(3),
            MmlEvent::Velocity(15),
            MmlEvent::Velocity(7),
        ];

        assert_eq!(get_highest_velocity(&events), 15);

        // Test with no velocity events
        let no_velocity_events = vec![MmlEvent::Tempo(120), MmlEvent::Octave(4)];
        assert_eq!(get_highest_velocity(&no_velocity_events), 0);
    }

    #[test]
    fn test_duration_consistency() {
        // This is a critical test for the main requirement:
        // MML track duration must equal MIDI track duration

        let ppq = 480;
        let smallest_unit = 64;

        // Create test MIDI notes with known durations
        let test_cases = vec![
            (480, 16), // Quarter note
            (240, 8),  // Eighth note
            (960, 32), // Half note
            (120, 4),  // Sixteenth note
        ];

        for (tick_duration, expected_smallest_units) in test_cases {
            let converted_units = tick_to_smallest_unit(tick_duration, ppq, smallest_unit);
            assert_eq!(
                converted_units, expected_smallest_units,
                "Failed for tick_duration={}, expected={}, got={}",
                tick_duration, expected_smallest_units, converted_units
            );

            // Test round-trip conversion should preserve duration
            let tick_per_unit = get_smallest_unit_in_tick(ppq, smallest_unit);
            let back_to_ticks = (converted_units as f32 * tick_per_unit).round() as usize;

            // Allow small rounding errors (within 1 tick)
            assert!(
                (back_to_ticks as isize - tick_duration as isize).abs() <= 1,
                "Round-trip conversion failed: {} ticks -> {} units -> {} ticks",
                tick_duration,
                converted_units,
                back_to_ticks
            );
        }
    }

    #[test]
    fn test_velocity_edge_cases() {
        // Test edge cases for velocity conversion

        // Test when min > max (invalid range) - should swap them
        let result = midi_velocity_to_mml_velocity(64, 10, 5);
        assert!(
            result >= 5 && result <= 10,
            "Result should be in valid range"
        );
        assert_eq!(result, 7); // Should use range 5-10, middle value around 7

        // Test when min == max
        assert_eq!(midi_velocity_to_mml_velocity(0, 7, 7), 7);
        assert_eq!(midi_velocity_to_mml_velocity(127, 7, 7), 7);

        // Test maximum MIDI velocity
        assert_eq!(midi_velocity_to_mml_velocity(127, 0, 15), 15);

        // Test minimum MIDI velocity
        assert_eq!(midi_velocity_to_mml_velocity(0, 0, 15), 0);
    }
}
