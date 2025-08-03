mod audio;
mod midi;
mod mml;
mod mml_player;
mod utils;

// https://github.com/fzyzcjy/flutter_rust_bridge/pull/1335/files
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "C" fn JNI_OnLoad(vm: jni::JavaVM, res: *mut std::os::raw::c_void) -> jni::sys::jint {
    use std::ffi::c_void;

    let vm = vm.get_java_vm_pointer() as *mut c_void;
    unsafe {
        ndk_context::initialize_android_context(vm, res);
    }
    jni::JNIVersion::V6.into()
}

pub use audio::{
    player::TrackPlayer,
    synth::{Synth, SynthOutputConnection},
};
pub use midi::event::NoteEvent;
pub use mml::parser::Parser;
pub use mml_player::{MmlPlayer, MmlPlayerOptions, NoteOnCallbackData, PlaybackStatus};
