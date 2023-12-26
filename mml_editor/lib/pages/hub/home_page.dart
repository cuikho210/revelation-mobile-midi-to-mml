import 'dart:io';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:flutter_remix_icon/flutter_remix_icon.dart';
import 'package:gap/gap.dart';
import 'package:file_picker/file_picker.dart';
import 'package:desktop_drop/desktop_drop.dart';
import 'package:mml_editor/native.dart';
import 'package:mml_editor/pages/editor/editor.dart';

class HomePage extends StatelessWidget {
	const HomePage({ super.key });

	@override
	Widget build(BuildContext context) {
		return DropTarget(
			onDragDone: (details) => onDragDone(details, context),
			child: Center(
				child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
					const Text("Drop a MIDI file here"),
					const Text("or"),
					const Gap(16),

					ElevatedButton.icon(
						icon: const Icon(RemixIcon.file_music_line),
						label: const Text("Import a MIDI file"),
						onPressed: () => pickFile(context),
					),
					const Gap(8),
					ElevatedButton.icon(
						icon: const Icon(RemixIcon.music_2_line),
						label: const Text("Download from Musescore"),
						onPressed: () => openDownloadFromMusescore(context),
					),
					const Gap(8),
				]),
			),
		);
	}

	onDragDone(DropDoneDetails details, BuildContext context) async {
		final path = details.files.first.path;

		if (context.mounted) {
			await parseMML(context, path);
		}
	}

	pickFile(BuildContext context) async {
		FilePickerResult? result = await FilePicker.platform.pickFiles(
			type: FileType.custom,
			allowedExtensions: ['mid'],
		);

		if (result == null) {
			if (context.mounted) {
				alert(context, "Canceled", "If you don't choose anymore, that's it :(");
			}
			return;
		}

		final path = result.files.first.path;
		if (path == null) {
			if (context.mounted) {
				alert(context, "Error", "Cannot get path");
			}
			return;
		}

		if (context.mounted) {
			await parseMML(context, path);
		}
	}

	parseMML(BuildContext context, String path) async {
		final bytes = await getBytesFromPath(path);

		try {
			final mmls = await api.parseMidi(bytes: bytes);

			if (context.mounted) {
				Navigator.push(
					context,
					MaterialPageRoute(
						builder: (context) => EditorPage(mmls: mmls)
					)
				);
			}
		} catch(e) {
			if (context.mounted) {
				alert(context, "Error", "Cannot parse this file!");
			}
		}
	}

	Future<Uint8List> getBytesFromPath(String path) async {
		final file = File(path);
		final bytes = await file.readAsBytes();

		return bytes;
	}

	openDownloadFromMusescore(BuildContext context) {
		alert(context, "Wait...", "Feature under development!");
	}

	alert(BuildContext context, String title, String content) {
		showDialog(context: context, builder: (context) => AlertDialog(
			title: Text(title),
			content: Text(content),
			actions: [
				TextButton(
					onPressed: () => Navigator.pop(context),
					child: const Text("Close")
				),
			],
		));
	}
}
