import 'package:flutter/services.dart';
import 'package:midi_to_mml/src/bindings/bindings.dart';

class SplitTrack {
  SplitTrack(int index) {
    SignalSplitTrackRequest(index: index).sendSignalToRust();
  }
}

class MergeTracks {
  MergeTracks(int indexA, int indexB) {
    SignalMergeTracksRequest(indexA: indexA, indexB: indexB).sendSignalToRust();
  }
}

class EqualizeTracks {
  EqualizeTracks(int indexA, int indexB) {
    SignalEqualizeTracksRequest(indexA: indexA, indexB: indexB)
        .sendSignalToRust();
  }
}

class RenameTrack {
  RenameTrack(int index, String name) {
    SignalRenameTrackRequest(index: index, name: name).sendSignalToRust();
  }
}

class SaveSongOptions {
  SaveSongOptions(SignalMmlSongOptions songOptions) {
    SignalUpdateMmlSongOptionsRequest(songOptions: songOptions)
        .sendSignalToRust();
  }
}

class PlaySong {
  PlaySong() {
    const SignalSetSongPlayStatusRequest(status: SignalPlayStatus.play)
        .sendSignalToRust();
  }
}

class PauseSong {
  PauseSong() {
    const SignalSetSongPlayStatusRequest(status: SignalPlayStatus.pause)
        .sendSignalToRust();
  }
}

class StopSong {
  StopSong() {
    const SignalSetSongPlayStatusRequest(status: SignalPlayStatus.stop)
        .sendSignalToRust();
  }
}

class LoadSoundfont {
  LoadSoundfont(Uint8List bytes) {
    const SignalLoadSoundfontRequest().sendSignalToRust(bytes);
  }

  LoadSoundfont.fromPath(String path) {
    loadSoundfontFromPath(path);
  }

  static loadSoundfontFromPath(String path) async {
    final bytes = await rootBundle.load(path);
    LoadSoundfont(bytes.buffer.asUint8List());
  }
}
