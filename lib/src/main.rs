extern crate revelation_mobile_midi_to_mml;
use revelation_mobile_midi_to_mml::{Song, SongOptions};

fn main() {
    let path = std::path::PathBuf::from("/home/cuikho210/Downloads/ban_khong_thuc_su_hanh_phuc.mid");
    let song = Song::from_path(
        path,
        // SongOptions::default(),
        SongOptions {
            auto_boot_velocity: false,
            velocity_min: 10,
            velocity_max: 15,
        },
    )
    .unwrap();

    let (left, right) = song.tracks.get(2).unwrap().split();
    let tracks = [left, right];

    for track in tracks.iter() {
        println!(
            "\nTrack {} - {} - {} notes ----------------------------------\n",
            track.name,
            track.instrument.name,
            track.mml_note_length,
        );
        println!("{}", track.to_mml());
    }
}
