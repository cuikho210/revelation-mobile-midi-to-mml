import 'dart:io';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:flutter_remix_icon/flutter_remix_icon.dart';
import 'package:gap/gap.dart';
import 'package:file_picker/file_picker.dart';
import 'package:desktop_drop/desktop_drop.dart';
import 'package:mml_editor/pages/convert_settings/convert_settings.dart';

class HomePage extends StatelessWidget {
	const HomePage({ super.key });

	@override
	Widget build(BuildContext context) {
		return DropTarget(
			onDragDone: (details) => onDragDone(details),
			child: Center(
				child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
					const Text("Drop a MIDI file here"),
					const Text("or"),
					const Gap(16),

					ElevatedButton.icon(
						icon: const Icon(RemixIcon.file_music_line),
						label: const Text("Import a MIDI file"),
						onPressed: () => pickFile(),
					),
				]),
			),
		);
	}

	onDragDone(DropDoneDetails details) async {
		final path = details.files.first.path;
		await parseMML(path);
	}

	pickFile() async {
		FilePickerResult? result = await FilePicker.platform.pickFiles(
			type: FileType.custom,
			allowedExtensions: ['mid'],
		);

		if (result == null) {
			return;
		}

		final path = result.files.first.path;
		if (path == null) {
			alert("Error", "Cannot get path");
			return;
		}

		await parseMML(path);
	}

	parseMML(String path) async {
		final bytes = await getBytesFromPath(path);

		try {
			final fileName = getFileNameFromPath(path);

			Get.to(ConvertSettings(
				fileName: fileName,
				midiBytes: bytes,
			), transition: Transition.rightToLeftWithFade);
		} catch(e) {
			alert("Error", "Cannot parse this file!");
		}
	}

	Future<Uint8List> getBytesFromPath(String path) async {
		final file = File(path);
		final bytes = await file.readAsBytes();

		return bytes;
	}

	alert(String title, String content) {
		Get.defaultDialog(
			title: title,
			content: Text(content),
		);
	}

	String getFileNameFromPath(String path) {
		// final regex = RegExp(r"[^~)('!*<>:;,?"*|\/\\]+\.mid");
		String regStr = r"[^~)('!*<>:;,?";
		regStr += r'"*|\/\\]+\.mid';
		final regex = RegExp(regStr);

		final match = regex.firstMatch(path);

		if (match != null && match[0] != null) {
			final name = match[0]!;
			return name.substring(0, name.length - 4);
		} else {
			return "";
		}
	}
}
