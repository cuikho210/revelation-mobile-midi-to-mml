use midi_to_mml::{Song, SongOptions};

fn main() {
    let path = std::path::PathBuf::from("/home/cuikho210/Downloads/_.mid");
    let song = Song::from_path(
        path,
        // SongOptions::default(),
        SongOptions {
            auto_boot_velocity: true,
            velocity_min: 11,
            velocity_max: 15,
        },
    )
    .unwrap();

    for track in song.tracks.iter() {
        println!("\n{} -------------------------------------\n", track.instrument.name);
        println!("{}", track.to_mml());
    }
}
