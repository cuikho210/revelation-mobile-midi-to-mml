import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/components/track.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/utils.dart';
import 'package:remixicon/remixicon.dart';

class DisplayMmls extends StatelessWidget {
	const DisplayMmls({ super.key });

	List<Widget> getTracks(AppController controller) {
		return controller.songStatus().tracks.map((track) {
			return TrackAndMml(
				track: track,
				mml: controller.mmls()[track.index],
			);
		}).toList();
	}

	@override
	Widget build(context) {
		final controller = Get.put(AppController());

		return Scaffold(
			appBar: AppBar(
				title: const Text("Export MML"),
				actions: [
					TextButton.icon(
						icon: const Icon(Remix.save_line),
						label: const Text("Save as file"),
						onPressed: () => SaveToTextFile(controller.exportMML()),
					),
				],
			),
			body: ListView(
				children: getTracks(controller),
			),
		);
	}
}
