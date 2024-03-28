import 'package:midi_to_mml/messages/types.pb.dart';

extension GetTitle on Track {
	String get title {
		return "$index. Track $name - $noteLength notes";
	}
}
