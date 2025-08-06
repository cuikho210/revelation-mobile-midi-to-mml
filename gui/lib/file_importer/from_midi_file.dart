import 'package:file_picker/file_picker.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/src/bindings/signals/signals.dart';
import 'package:midi_to_mml/utils.dart';
import 'package:path/path.dart';

class FromMidiFile {
  static Future pickFile() async {
    final path = await getMidiFilePath();
    open(path);
  }

  static Future open(String? path) async {
    if (path == null) {
      AlertMessage.error("Cannot open this file!");
      return;
    }

    final AppController controller = Get.find();

    controller.fileName(basename(path));
    SignalLoadSongFromPathRequest(path: path).sendSignalToRust();
  }

  /// Open the file picker and return the path of file
  static Future<String?> getMidiFilePath() async {
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
