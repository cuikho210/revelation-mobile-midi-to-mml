import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:flutter_remix_icon/flutter_remix_icon.dart';
import 'package:gap/gap.dart';
import 'package:get/get.dart';
import 'package:mml_editor/native.dart';
import 'package:mml_editor/pages/editor/editor.dart';
import './controller.dart';

class ConvertSettings extends StatelessWidget {
	final String fileName;
	final Uint8List midiBytes;

	const ConvertSettings({
		super.key,
		required this.fileName,
		required this.midiBytes,
	});

	void nextToEditor(bool isAutoSplit) async {
		List<String> mmls = [];

		if (isAutoSplit) {
			mmls = await api.parseMidi(
				bytes: midiBytes,
				isAutoSplit: true,
				toMerge: [],
			);
		} else {
			mmls = await api.parseMidi(
				bytes: midiBytes,
				isAutoSplit: false,
				toMerge: [],
			);
		}

		Get.to(EditorPage(
			fileName: fileName,
			mmls: mmls,
			midiBytes: midiBytes,
		));
	}

	@override
	Widget build(context) {
		var c = Get.put(Controller());

		return Scaffold(
			appBar: AppBar(
				title: Text("Options: $fileName"),
				leading: IconButton(
					icon: const Icon(RemixIcon.arrow_left_s_line),
					onPressed: () => Get.back(),
					padding: const EdgeInsets.all(12),
					tooltip: "Back",
				),
				actions: [
					TextButton.icon(
						icon: const Icon(RemixIcon.arrow_right_s_line),
						label: const Text("Next"),
						onPressed: () => nextToEditor(c.isAutoSplit()),
					),
				],
			),
			body: ListView(
				padding: const EdgeInsets.all(16),
				children: [
					Obx(() => AutoSplitSetting(
						value: c.isAutoSplit(),
						onChanged: c.setIsAutoSplit,
					)),

					const Gap(16),
					Divider(thickness: 1, height: 1, color: Theme.of(context).colorScheme.primary.withAlpha(64),),
					const Gap(16),

					Obx(() => MergeTracks(
						isVisible: !c.isAutoSplit(),
					)),
				],
			),
		);
	}
}

class AutoSplitSetting extends StatelessWidget {
	final bool value;
	final void Function(bool? value) onChanged;

	const AutoSplitSetting({
		super.key,
		required this.value,
		required this.onChanged,
	});

	@override
	Widget build(context) {
		return Row(children: [
			Checkbox(
				value: value,
				onChanged: onChanged,
			),
			const Gap(8),
			const Text("Auto split track"),
		]);
	}
}

class MergeTracks extends StatelessWidget {
	final bool isVisible;

	const MergeTracks({
		super.key,
		required this.isVisible,
	});

	@override
	Widget build(context) {
		return Visibility(
			visible: isVisible,
			child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
				Text("Merge Tracks", style: Theme.of(context).textTheme.titleLarge),
				const Text("Drag and drop to merge tracks"),
			]),
		);
	}
}
