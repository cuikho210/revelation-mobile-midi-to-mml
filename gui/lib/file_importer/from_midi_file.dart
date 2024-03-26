import 'package:file_picker/file_picker.dart';
import 'package:midi_to_mml/messages/commands.pb.dart';
import 'package:midi_to_mml/utils.dart';

class FromMidiFile {
	FromMidiFile.open(String path) {
		open(path);
	}

	FromMidiFile.pickFile() {
		pickFile();
	}

	Future pickFile() async {
		final path = await getMidiFilePath();
		open(path);
	}

	Future open(String? path) async {
		if (path == null) {
			AlertError("Cannot open this file!");
			return;
		}

		ImportMidiData(path: path).sendSignalToRust(null);
	}

	/// Open the file picker and return the path of file
	Future<String?> getMidiFilePath() async {
		FilePickerResult? result = await FilePicker.platform.pickFiles(
			type: FileType.custom,
			allowedExtensions: ['mid', 'midi'],
		);

		if (result != null) {
			return result.files.single.path;
		} else {
			return null;
		}
	}
}
