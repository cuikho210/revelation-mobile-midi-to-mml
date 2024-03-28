import 'dart:io';
import 'dart:typed_data';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

class AlertError {
	AlertError(String content) {
		if (Get.context != null) {
			ScaffoldMessenger.of(Get.context!).showSnackBar(SnackBar(
				content: Text(content),
				backgroundColor: Theme.of(Get.context!).colorScheme.error,
			));
		}	
	}
}

class SaveToTextFile {
	SaveToTextFile({
		required String fileName,
		required String content,
	}) {
		fileName = "$fileName.txt";

		if (Platform.isAndroid || Platform.isIOS) {
			FilePicker.platform.saveFile(
				dialogTitle: "Save MML to a text file",
				fileName: fileName,
				bytes: Uint8List.fromList(content.codeUnits),
			);
		} else {
			FilePicker.platform.saveFile(
				dialogTitle: "Save MML to a text file",
				fileName: fileName,
				lockParentWindow: true,
			).then((path) {
				if (path != null) {
					final file = File(path);
					file.writeAsString(content);
				}
			});
		}
	}
}
