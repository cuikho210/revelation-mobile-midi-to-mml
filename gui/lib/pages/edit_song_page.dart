import 'package:flutter/material.dart';
import 'package:gap/gap.dart';
import 'package:get/get.dart';
import 'package:remixicon/remixicon.dart';
import 'package:midi_to_mml/components/track.dart';
import 'package:midi_to_mml/controller.dart';

class EditSongPage extends StatelessWidget {
	const EditSongPage({ super.key });

	@override
	Widget build(context) {
		return Scaffold(
			appBar: AppBar(
				title: const Text("Edit"),
				actions: [
					TextButton.icon(
						icon: const Icon(Remix.export_line),
						label: const Text("Export"),
						onPressed: () => {},
					),
				],
			),
			body: ListView(children: const [
				_Options(),
				Gap(32),
				_Tracks(),
			]),
		);
	}
}

class _Tracks extends GetView<AppController> {
	const _Tracks();

	List<Widget> getTrackWidgets() {
		return controller.songStatus().tracks.map((track) => TrackListTitle(
			trackIndex: track.index,
			trackName: track.name,
			instrumentName: track.instrumentName,
			trackNoteLength: track.noteLength,
		)).toList();
	}

	@override
	Widget build(context) {
		return Obx(() =>Column(children: [
			Text("Tracks", style: Theme.of(context).textTheme.titleLarge),
			const Gap(16),
			
			...getTrackWidgets()
		]));
	}
}

class _Options extends GetView<AppController> {
	const _Options();

	@override
	Widget build(context) {
		return Column(children: [
			Text("Song options", style: Theme.of(context).textTheme.titleLarge),
			const Gap(16),

			Obx(() => CheckboxListTile(
				title: const Text("Auto boot velocity"),
				value: controller.songStatus().options.autoBootVelocity,
				onChanged: (newValue) {
					controller.songStatus.value.options.autoBootVelocity = (newValue == true);
					controller.songStatus.refresh();
				},
			)),

			ListTile(
				title: const Text("Velocity min"),
				trailing: SizedBox(
					width: 48,
					child: Obx(() => TextFormField(
						textAlign: TextAlign.end,
						initialValue: controller.songStatus().options.velocityMin.toString(),
						keyboardType: TextInputType.number,
						onChanged: (newValue) {
							controller.songStatus.value.options.velocityMin = int.parse(newValue);
						},
					)),
				),
			),

			ListTile(
				title: const Text("Velocity max"),
				trailing: SizedBox(
					width: 48,
					child: Obx(() => TextFormField(
						textAlign: TextAlign.end,
						initialValue: controller.songStatus().options.velocityMax.toString(),
						keyboardType: TextInputType.number,
						onChanged: (newValue) {
							controller.songStatus.value.options.velocityMax = int.parse(newValue);
						},
					)),
				),
			),
		]);
	}
}
