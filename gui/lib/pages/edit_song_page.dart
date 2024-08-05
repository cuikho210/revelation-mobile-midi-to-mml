import 'package:flutter/material.dart';
import 'package:gap/gap.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/messages/rust_to_dart.pb.dart';
import 'package:remixicon/remixicon.dart';
import 'package:midi_to_mml/components/track.dart';
import 'package:midi_to_mml/controller.dart';

class EditSongPage extends StatelessWidget {
	const EditSongPage({ super.key });

	@override
	Widget build(context) {
		final controller = Get.put(AppController());
		listenUpdateMmlTracksStream(controller);

		return Scaffold(
			appBar: AppBar(
				title: Text("Edit: ${controller.fileName()}"),
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

	void listenUpdateMmlTracksStream(AppController controller) async {
		await for (final signal in SignalUpdateMmlTracks.rustSignalStream) {
			WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
				controller.tracks(signal.message.tracks);
			});
		}
	}
}

class _Tracks extends GetView<AppController> {
	const _Tracks();

	List<Widget> getTrackWidgets() {
		return controller.tracks().map((track) => TrackListTitle(track: track)).toList();
	}

	@override
	Widget build(context) {
		return Obx(() => Column(children: [
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
				value: controller.songOptions().autoBootVelocity,
				onChanged: (newValue) {
					controller.songOptions.value.autoBootVelocity = (newValue == true);
					controller.songOptions.refresh();
				},
			)),

			ListTile(
				title: const Text("Velocity min"),
				trailing: SizedBox(
					width: 48,
					child: Obx(() => TextFormField(
						textAlign: TextAlign.end,
						initialValue: controller.songOptions().velocityMin.toString(),
						keyboardType: TextInputType.number,
						onChanged: (newValue) {
							controller.songOptions().velocityMin = int.parse(newValue);
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
						initialValue: controller.songOptions().velocityMax.toString(),
						keyboardType: TextInputType.number,
						onChanged: (newValue) {
							controller.songOptions().velocityMax = int.parse(newValue);
						},
					)),
				),
			),
		]);
	}
}
