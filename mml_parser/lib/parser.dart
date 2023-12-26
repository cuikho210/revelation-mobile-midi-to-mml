
const int defaultTempo = 120;
const int defaultNoteLength = 4;
const int defaultOctave = 4;

class Parser {
	Parser();

	parse(String mmlString) {
		var regExp = RegExp(
			r"([abcdefg]{1}[+|-]?[0-9]*[.]*)|([ovt][0-9]+)|([<>])|(r[0-9]*[.]*)|(#.*$)",
			multiLine: true,
		);

		var matches = regExp.allMatches(mmlString);
		print(matches);
	}
}
