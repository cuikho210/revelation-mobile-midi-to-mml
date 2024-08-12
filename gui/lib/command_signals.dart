import 'package:flutter/services.dart';
import 'package:midi_to_mml/messages/dart_to_rust.pb.dart';
import 'package:midi_to_mml/messages/types.pb.dart';

class SplitTrack {
	SplitTrack(int index) {
		SignalSplitTrackPayload(index: index).sendSignalToRust();
	}
}

class MergeTracks {
	MergeTracks(int indexA, int indexB) {
		SignalMergeTracksPayload(indexA: indexA, indexB: indexB).sendSignalToRust();
	}
}

class EqualizeTracks {
	EqualizeTracks(int indexA, int indexB) {
		SignalEqualizeTracksPayload(indexA: indexA, indexB: indexB).sendSignalToRust();
	}
}

class RenameTrack {
	RenameTrack(int index, String name) {
		SignalRenameTrackPayload(index: index, name: name).sendSignalToRust();
	}
}

class SaveSongOptions {
	SaveSongOptions(SignalMmlSongOptions songOptions) {
		SignalUpdateMmlSongOptionsPayload( songOptions: songOptions ).sendSignalToRust();
	}
}

class PlaySong {
	PlaySong() {
		SignalSetSongPlayStatusPayload( status: SignalPlayStatus.PLAY ).sendSignalToRust();
	}
}

class PauseSong {
	PauseSong() {
		SignalSetSongPlayStatusPayload( status: SignalPlayStatus.PAUSE ).sendSignalToRust();
	}
}

class StopSong {
	StopSong() {
		SignalSetSongPlayStatusPayload( status: SignalPlayStatus.STOP ).sendSignalToRust();
	}
}

class LoadSoundfont {
	LoadSoundfont(Uint8List bytes) {
		SignalLoadSoundfontPayload().sendSignalToRust(bytes);
	}
}

