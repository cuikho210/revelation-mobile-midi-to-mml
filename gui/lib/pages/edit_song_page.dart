import 'package:flutter/material.dart';
import 'package:gap/gap.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/command_signals.dart';
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
				title: Text(controller.fileName(), style: Theme.of(context).textTheme.titleMedium),
				actions: [
					TextButton.icon(
						icon: const Icon(Remix.export_line),
						label: const Text("Export"),
						onPressed: () => {},
					),
				],
			),
			body: ListView(children: const [
				_SongControls(),
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

class _SongControls extends GetView<AppController> {
	const _SongControls();

	Future<void> showSettingsModal(BuildContext context) async {
		return showModalBottomSheet<void>(context: context, builder: (context) {
			final smallestUnitValues = [16, 32, 64, 128, 256];
			var smallestUnitIndex = smallestUnitValues.indexOf(controller.songOptions().smallestUnit).obs;

			return SizedBox(
				height: 512,
				child: Column(children: [
					Container(padding: const EdgeInsets.all(16), child: Flex(
						direction: Axis.horizontal,
						mainAxisAlignment: MainAxisAlignment.spaceBetween,
						children: [
							Row(children: [
								const Icon(Remix.settings_line),
								const Gap(8),
								Text("Song Settings", style: Theme.of(context).textTheme.titleMedium),
							]),
							ElevatedButton.icon(
								label: const Text("Apply"),
								icon: const Icon(Remix.save_line),
								onPressed: () {
									SaveSongSettings(controller.songOptions());
									Navigator.pop(context);
								},
							),
						],
					)),

					Expanded(child: ListView(children: [
						Obx(() => CheckboxListTile(
							title: const Text("Auto boot velocity"),
							value: controller.songOptions().autoBootVelocity,
							onChanged: (newValue) {
								controller.songOptions.value.autoBootVelocity = (newValue == true);
								controller.songOptions.refresh();
							},
						)),
						
						Obx(() => CheckboxListTile(
							title: const Text("Auto equalize note length"),
							value: controller.songOptions().autoEqualizeNoteLength,
							onChanged: (newValue) {
								controller.songOptions.value.autoEqualizeNoteLength = (newValue == true);
								controller.songOptions.refresh();
							},
						)),

						ListTile(
							title: const Text("Velocity min"),
							subtitle: Obx(() => Slider(
								value: controller.songOptions().velocityMin.toDouble(),
								label: controller.songOptions().velocityMin.toString(),
								min: 0,
								max: 15,
								divisions: 15,
								onChanged: (value) {
									controller.songOptions().velocityMin = value.toInt();
									controller.songOptions.refresh();
								}
							)),
						),
						
						ListTile(
							title: const Text("Velocity max"),
							subtitle: Obx(() => Slider(
								value: controller.songOptions().velocityMax.toDouble(),
								label: controller.songOptions().velocityMax.toString(),
								min: 0,
								max: 15,
								divisions: 15,
								onChanged: (value) {
									controller.songOptions().velocityMax = value.toInt();
									controller.songOptions.refresh();
								}
							)),
						),

						ListTile(
							title: const Text("Min gap for chord"),
							subtitle: Obx(() => Slider(
								value: controller.songOptions().minGapForChord.toDouble(),
								label: controller.songOptions().minGapForChord.toString(),
								min: 0,
								max: 16,
								divisions: 16,
								onChanged: (value) {
									controller.songOptions().minGapForChord = value.toInt();
									controller.songOptions.refresh();
								}
							)),
						),

						ListTile(
							title: const Text("Smallest unit"),
							subtitle: Obx(() => Slider(
								value: smallestUnitIndex().toDouble(),
								label: controller.songOptions().smallestUnit.toString(),
								min: 0,
								max: smallestUnitValues.length - 1,
								divisions: smallestUnitValues.length - 1,
								onChanged: (newIndex) {
									smallestUnitIndex(newIndex.toInt());
									final value = smallestUnitValues[smallestUnitIndex()];
									controller.songOptions().smallestUnit = value;
									controller.songOptions.refresh();
								}
							)),
						),
					])),
				]),
			);
		});
	}

	@override
	Widget build(context) {
		return Flex(
			direction: Axis.horizontal,
			mainAxisAlignment: MainAxisAlignment.spaceBetween,
			children: [
				TextButton.icon(
					onPressed: () => showSettingsModal(context),
					label: const Text("Song Settings"),
					icon: const Icon(Remix.settings_line),
				),
				Row(children: [
					IconButton(
						onPressed: () => (),
						icon: const Icon(Remix.play_line),
					),
					IconButton(
						onPressed: () => (),
						icon: const Icon(Remix.stop_line),
					),
				])
			],
		);
	}
}
