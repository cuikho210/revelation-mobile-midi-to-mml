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
		), transition: Transition.rightToLeft);
	}

	@override
	Widget build(context) {
		var c = Controller(midiBytes);

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
					TextButton(
						child: const Text("Next"),
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
						key: c.refreshKey(),
						isVisible: !c.isAutoSplit(),
						tracksList: c.tracks,
						merge: c.mergeTracks,
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
		return CheckboxListTile(
			title: const Text("Auto split track"),
			value: value,
			onChanged: onChanged,
		);
	}
}

class MergeTracks extends StatelessWidget {
	final bool isVisible;
	final List<List<int>> tracksList;
	final Function(int to, int from, int value) merge;

	const MergeTracks({
		super.key,
		required this.isVisible,
		required this.tracksList,
		required this.merge,
	});

	List<Widget> getColums() {
		List<Widget> cols = [];

		tracksList.asMap().forEach((index, tracks) {
			final children = getColumChildren(index, tracks);
			final column = Column(children: children);

			final dragTarget = DragTarget<(int, int)>(
				builder: (
					BuildContext ctx,
					List<dynamic> accepted,
					List<dynamic> rejected
				) => column,
				onAccept: ((int, int) data) {
					if (tracks.length > 1) return;

					var to = tracks.first;
					var from = data.$1;
					var value = data.$2;
					merge(to, from, value);
				},
			);

			cols.add(dragTarget);
		});

		return cols;
	}

	List<Widget> getColumChildren(int colIndex, List<int> col) {
		List<Widget> children = [];

		for (final i in col) {
			final child = Container(
				color: Get.theme.colorScheme.primary.withAlpha(64),
				margin: const EdgeInsets.all(1),
				padding: const EdgeInsets.all(8),
				child: Text("Track $i", style: Get.theme.textTheme.bodyLarge),
			);

			final dragable = Draggable<(int, int)>(
				data: (colIndex, i),
				feedback: child,
				child: child,
			);

			children.add(dragable);
		}

		return children;
	}

	@override
	Widget build(context) {
		return Visibility(
			visible: isVisible,
			child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
				Text("Merge Tracks", style: Theme.of(context).textTheme.titleLarge),
				const Text("Drag and drop to merge tracks"),
				const Gap(16),
				SizedBox(
					height: 256,
					child: ListView(
						scrollDirection: Axis.horizontal,
						children: getColums(),
					),
				),
			]),
		);
	}
}
