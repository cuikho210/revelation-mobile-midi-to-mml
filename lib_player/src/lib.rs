mod synth;
mod parser;
mod note_event;
mod mml_event;
mod utils;
mod mml_player;
mod track_player;

// https://github.com/fzyzcjy/flutter_rust_bridge/pull/1335/files
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn JNI_OnLoad(vm: jni::JavaVM, res: *mut std::os::raw::c_void) -> jni::sys::jint {
    use std::ffi::c_void;

    let vm = vm.get_java_vm_pointer() as *mut c_void;
    unsafe {
        ndk_context::initialize_android_context(vm, res);
    }
    jni::JNIVersion::V6.into()
}

pub use synth::Synth;
pub use synth::SynthOutputConnection;

pub use mml_player::MmlPlayer;
pub use mml_player::MmlPlayerOptions;
pub use mml_player::NoteOnCallbackData;
pub use mml_player::PlaybackStatus;

pub use note_event::NoteEvent;
pub use parser::Parser;
pub use track_player::TrackPlayer;
