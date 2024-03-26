import 'package:file_picker/file_picker.dart';
import 'package:midi_to_mml/messages/commands.pb.dart';
import 'package:midi_to_mml/utils.dart';

class FromMidiFile {
	FromMidiFile() {
		init();
	}

	Future init() async {
		final filePath = await getMidiFilePath();

		if (filePath == null) {
			AlertError("Cannot open this file!");
			return;
		}

		ImportMidiData(path: filePath).sendSignalToRust(null);
	}

	/// Open the file picker and return the path of file
	Future<String?> getMidiFilePath() async {
		FilePickerResult? result = await FilePicker.platform.pickFiles();

		if (result != null) {
			return result.files.single.path;
		} else {
			return null;
		}
	}
}
