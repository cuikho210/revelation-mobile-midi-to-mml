import './events.dart';

class Parser {
	Parser();

	List<dynamic> parse(String mmlString) {
		var matches = matchEvents(mmlString);
		var events = parseEvents(matches);

		return events;
	}

	Iterable<Match> matchEvents(String mml) {
		var regex = r"(t[0-9]+)"; // Tempo
		regex += r"|(v[0-9]+)"; // Velocity
		regex += r"|(o[0-9])"; // Set octave
		regex += r"|(>)"; // Incre octave
		regex += r"|(<)"; // Decre octave
		regex += r"|(:)"; // Connect chord
		regex += r"|(r[0-9]+(\.)?((&r[0-9]+(\.)?)+)?)"; // Set rest
		regex += r"|([abcdefg][+-]?[0-9]+(\.)?((&[abcdefg][+-]?[0-9]+(\.)?)+)?)"; // Set note

		var regExp = RegExp(
			regex,
			multiLine: true,
			caseSensitive: false,
		);

		return regExp.allMatches(mml);
	}

	List<dynamic> parseEvents(Iterable<Match> matches) {
		List<dynamic> result = [];
		int currentOctave = defaultOctave;
		int currentPosition = 0;
		bool isConnectingToChord = false;

		for (dynamic match in matches) {
			List<String?> groups = match.groups([1, 2, 3, 4, 5, 6, 7, 12]);

			if (groups[0] != null) {
				result.add(
					SetTempo(groups[0]!)
				);
			} else if (groups[1] != null) {
				result.add(
					SetVelocity(groups[1]!)
				);
			} else if (groups[2] != null) {
				var octave = SetOctave(groups[2]!);
				result.add(octave);
				currentOctave = octave.value;
			} else if (groups[3] != null) {
				result.add(IncreOctave());
			} else if (groups[4] != null) {
				result.add(DecreOctave());
			} else if (groups[5] != null) {
				result.add(ConnectChord());
				isConnectingToChord = true;
			} else if (groups[6] != null) {
				var rest = SetRest(groups[6]!);
				currentPosition += rest.duration;
				result.add(rest);
			} else if (groups[7] != null) {
				var note = SetNote(
					rawString: groups[7]!,
					octave: currentOctave,
					position: currentPosition,
				);

				if (!isConnectingToChord) {
					currentPosition += note.duration;
				} else {
					isConnectingToChord = false;
				}

				result.add(note);
			} else {
				print("Match failed");
			}
		}

		return result;
	}
}
