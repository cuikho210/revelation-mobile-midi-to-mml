import 'package:midi_to_mml/messages/types.pb.dart';

extension GetTitle on SignalMmlTrack {
	String get title {
		return "$index. Track $name - $mmlNoteLength notes";
	}
}
