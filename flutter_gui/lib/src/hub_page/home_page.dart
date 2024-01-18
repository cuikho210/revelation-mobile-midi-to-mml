import "dart:developer";

import "package:desktop_drop/desktop_drop.dart";
import "package:file_picker/file_picker.dart";
import "package:flutter/material.dart";
import "package:flutter_remix_icon/flutter_remix_icon.dart";
import "package:gap/gap.dart";
import "package:get/get.dart";
import "package:flutter_gui/src/utils.dart";
import 'package:flutter_gui/src/rust/api/simple.dart';

class HomePage extends StatelessWidget {
	final utils = const Utils();

	const HomePage({ super.key });

	@override
	Widget build(context) {
		return DropTarget(
			onDragDone: onDragDone,
			child: Column(
				mainAxisAlignment: MainAxisAlignment.center,
				children: [
					const Text("Drop a MIDI file here"),
					const Text("or"),
					const Gap(16),

					ElevatedButton.icon(
						icon: const Icon(RemixIcon.file_music_line),
						label: const Text("Import a MIDI file"),
						onPressed: pickFile,
					)
				],
			),
		);
	}

	onDragDone(DropDoneDetails details) async {
		final path = details.files.first.path;
		await parseToMML(path);
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
			utils.alert("Error", "Cannot get path");
			return;
		}

		await parseToMML(path);
	}

	parseToMML(String path) async {
		final bytes = await utils.getBytesFromPath(path);
		final fileName = utils.getFileNameFromPath(path);

		try {
			final song = parseMidiToMml(bytes: bytes);
			inspect(song);
			// Get.to(ConvertSettings(
			// 	fileName: fileName,
			// 	midiBytes: bytes,
			// ), transition: Transition.rightToLeftWithFade);
			
		} catch(e) {
			utils.alert("Error", "Cannot parse this file!");
		}
	}
}
