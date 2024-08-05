import 'package:midi_to_mml/messages/dart_to_rust.pb.dart';

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
