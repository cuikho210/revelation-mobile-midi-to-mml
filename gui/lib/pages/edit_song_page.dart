import 'package:flutter/material.dart';
import 'package:gap/gap.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/command_signals.dart';
import 'package:midi_to_mml/pages/display_mmls.dart';
import 'package:remixicon/remixicon.dart';
import 'package:midi_to_mml/components/track.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/messages/rust_to_dart.pb.dart';

class EditSongPage extends StatelessWidget {
	const EditSongPage({ super.key });

	@override
	Widget build(context) {
		final controller = Get.put(AppController());

		listenToMmlSignalStream(controller);
		listenSplitTrackSignalStream(controller);
		listenMergeTracksSignalStream(controller);

		return Scaffold(
			appBar: AppBar(
				title: const Text("Edit"),
				actions: [
					TextButton.icon(
						icon: const Icon(Remix.export_line),
						label: const Text("Export"),
						onPressed: () => ExportToMML(controller.songStatus().options),
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

	void listenToMmlSignalStream(AppController controller) async {
		await for (final signal in GetMMLOutput.rustSignalStream) {
			WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
				final mmls = signal.message.mml;
				controller.mmls(mmls);
				Get.to(const DisplayMmls());
			});
		}
	}

	void listenMergeTracksSignalStream(AppController controller) async {
		await for (final signal in MergeTracksOutput.rustSignalStream) {
			WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
				final tracks = signal.message.tracks;
				controller.songStatus.value.tracks.clear();
				controller.songStatus.value.tracks.addAll(tracks);
				controller.songStatus.refresh();
			});
		}
	}

	void listenSplitTrackSignalStream(AppController controller) async {
		await for (final signal in SplitTrackOutput.rustSignalStream) {
			WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
				final tracks = signal.message.tracks;
				controller.songStatus.value.tracks.clear();
				controller.songStatus.value.tracks.addAll(tracks);
				controller.songStatus.refresh();
			});
		}
	}
}

class _Tracks extends GetView<AppController> {
	const _Tracks();

	List<Widget> getTrackWidgets() {
		return controller.songStatus().tracks.map((track) => TrackListTitle(track: track)).toList();
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
