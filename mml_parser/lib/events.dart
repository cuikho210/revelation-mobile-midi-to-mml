import './utils.dart';

const int defaultTempo = 120;
const int defaultDuration = 64 ~/ 4; // Quarter note
const int defaultOctave = 4;
const int defaultVelocity = 12;

enum PitchClass {
	C,
	Db, D,
	Eb, E,
	F,
	Gb, G,
	Ab, A,
	Bb, B
}

class SetTempo {
	final String rawString;
	final int value = defaultTempo;
	const SetTempo(this.rawString);
}

class SetVelocity {
	final String rawString;
	final int value = defaultVelocity;
	const SetVelocity(this.rawString);
}

class SetOctave {
	String rawString;
	int value = defaultOctave;

	SetOctave(this.rawString) {
		value = int.parse(rawString.substring(1));
	}
}

class IncreOctave {}
class DecreOctave {}
class ConnectChord {}

class SetRest {
	String rawString;
	int duration = defaultDuration;

	SetRest(this.rawString) {
		duration = Utils().getDurationFromRawString(rawString);
	}
}

class SetNote {
	String rawString;
	late int duration;
	int position;
	int octave;
	PitchClass pitchClass = PitchClass.C;
	int velocity = defaultVelocity;

	SetNote({
		required this.rawString,
		required this.octave,
		required this.position,
	}) {
		duration = Utils().getDurationFromRawString(rawString);
	}
}
