import 'package:midi_to_mml/messages/commands.pb.dart';

class SplitTrack {
	SplitTrack(int index) {
		Split(index: index).sendSignalToRust(null);
	}
}

class MergeTracks {
	MergeTracks(int indexA, int indexB) {
		Merge(indexA: indexA, indexB: indexB).sendSignalToRust(null);
	}
}
