
class Utils {
	int getDurationFromRawString(String input) {
		int result = 0;
		final notes = input.split("&");

		for (final note in notes) {
			bool hasDot = false;
			if (note.endsWith(".")) {
				hasDot = true;
			}

			int noteDuration = int.parse(note.replaceAll(
				RegExp(r"[^0-9]+"),
				"",
			));

			final int durationInNote64 = 64 ~/ noteDuration;
			if (hasDot) {
				result += durationInNote64 * 2;
			} else {
				result += durationInNote64;
			}
		}

		return result;
	}
}

