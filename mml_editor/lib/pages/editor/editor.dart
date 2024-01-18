import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import './header.dart';
import './tab_selector.dart';
import '../display_mml_output.dart';
import '././editor_controller.dart';

class EditorPage extends StatelessWidget {
	final String fileName;
	final List<String> mmls;
	final Uint8List midiBytes;

	const EditorPage({
		super.key,
		required this.fileName,
		required this.mmls,
		required this.midiBytes,
	});

	@override
	Widget build(context) {
		var controller = EditorController(midiBytes, mmls);

		return Scaffold(
			body: Column(children: [
				EditorHeader(
					fileName: fileName,
				),
				Obx(() => TabSelector(
					key: controller.refreshKey(),
					selectedTabIndex: controller.currentTabIndex(),
					tracks: controller.tracks,
					updateTabIndex: (index) => controller.updateCurrentTabIndex(index),
					togglePlayStatus: (index) => controller.toggleTrackPlayStatus(index),
				)),
				Obx(() => Expanded(child: ListView(
					padding: const EdgeInsets.all(16),
					children: [
						TrackView(
							index: controller.currentTabIndex(),
							content: controller.getCurrentTrackContent(),
						),
					],
				))),
			]),
		);
	}
}
