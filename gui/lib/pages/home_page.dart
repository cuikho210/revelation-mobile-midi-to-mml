import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/pages/edit_song_page.dart';
import 'package:remixicon/remixicon.dart';
import 'package:midi_to_mml/file_importer/from_midi_file.dart';
import 'package:midi_to_mml/messages/rust_to_dart.pb.dart';
import 'package:gap/gap.dart';
import 'package:midi_to_mml/utils.dart';
import 'package:midi_to_mml/controller.dart';

class HomePage extends StatelessWidget {
	const HomePage({ super.key });

	@override
	Widget build(context) {
		final controller = Get.put(AppController());

		return Scaffold(
			appBar: AppBar(
				title: const _AppTitle(),
			),
			body: Center(child: Column(
				mainAxisAlignment: MainAxisAlignment.center,
				children: [
					ElevatedButton.icon(
						onPressed: () => FromMidiFile(),
						icon: const Icon(Remix.file_music_line),
						label: const Text("Import a MIDI file"),
					),

					const Gap(16),
					StreamBuilder(
						stream: ImportMidiDataOutput.rustSignalStream,
						builder: (context, snapshot) {
							final signal = snapshot.data;

							if (signal != null) {
								final message = signal.message;

								if (message.isOk) {
									WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
										controller.songStatus(message.songStatus);
										Get.to(const EditSongPage());
									});
								} else {
									WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
										const Utils().alertError("Incorrect MIDI file!");
									});
								}
							}

							return const Text("Drop a file here");
						},
					),
				],
			)),
		);
	}
}

class _AppTitle extends GetView<AppController> {
	const _AppTitle();

	@override
	Widget build(context) {
		return Obx(() => Text("MIDI to MML ${controller.packageInfo().version}"));
	}
}
