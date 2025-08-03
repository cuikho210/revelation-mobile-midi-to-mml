import 'package:midi_to_mml/src/bindings/bindings.dart';

extension GetTitle on SignalMmlTrack {
  String get title {
    return "$index. Track $name - $mmlNoteLength notes";
  }
}
