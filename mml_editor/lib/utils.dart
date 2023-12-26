import '././config.dart';

class Utils {
	int getNoteWidthFromDuration(int duration) {
		return duration * note64Width;
	}
}
