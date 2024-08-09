import 'package:file_picker/file_picker.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/messages/dart_to_rust.pb.dart';
import 'package:midi_to_mml/utils.dart';
import 'package:path/path.dart';

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
			AlertMessage.error("Cannot open this file!");
			return;
		}

		final controller = Get.put(AppController());

		controller.fileName(basename(path));
		SignalLoadSongFromPathPayload(path: path).sendSignalToRust();
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
