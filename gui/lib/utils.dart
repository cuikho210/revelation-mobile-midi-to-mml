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
	SaveToTextFile(String content) {
		FilePicker.platform.saveFile(
			dialogTitle: "Save MML to a text file",
			fileName: "exported_mml.txt",
			bytes: Uint8List.fromList(content.codeUnits),
		);
	}
}
