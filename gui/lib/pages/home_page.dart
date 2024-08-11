import 'package:desktop_drop/desktop_drop.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/command_signals.dart';
import 'package:midi_to_mml/messages/rust_to_dart.pb.dart';
import 'package:midi_to_mml/messages/types.pb.dart';
import 'package:midi_to_mml/pages/edit_song_page.dart';
import 'package:midi_to_mml/utils.dart';
import 'package:remixicon/remixicon.dart';
import 'package:midi_to_mml/file_importer/from_midi_file.dart';
import 'package:gap/gap.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:url_launcher/url_launcher.dart';

class HomePage extends StatelessWidget {
	const HomePage({ super.key });

	toEditPage(AppController controller) async {
		await Get.to(const EditSongPage());

		StopSong();
		controller.playbackStatus(SignalPlayStatus.STOP);
	}

	@override
	Widget build(context) {
		final controller = Get.put(AppController());

		return Scaffold(
			appBar: AppBar(
				title: const _AppTitle(),
				actions: [
					IconButton(
						icon: const Icon(Remix.github_line),
						onPressed: () {
							final uri = Uri.parse("https://github.com/cuikho210/revelation-mobile-midi-to-mml");
							launchUrl(uri);
						},
					),
					TextButton.icon(
						icon: const Icon(Remix.hand_heart_line),
						label: const Text("Donate"),
						onPressed: () {
							final uri = Uri.parse("https://github.com/cuikho210/revelation-mobile-midi-to-mml/blob/main/README/DONATE.md");
							launchUrl(uri);
						},
					)
				],
			),
			body: DropTarget(
				onDragDone: (detail) {
					final path = detail.files.first.path;
					FromMidiFile.open(path);
				},
				child: Center(child: Column(
					mainAxisAlignment: MainAxisAlignment.center,
					children: [
						ElevatedButton.icon(
							onPressed: () => FromMidiFile.pickFile(),
							icon: const Icon(Remix.file_music_line),
							label: const Text("Import a MIDI file"),
						),

						const Gap(16),

						StreamBuilder(
							stream: SignalLoadSongFromPathResponse.rustSignalStream,
							builder: (context, snapshot) {
								final signal = snapshot.data;

								if (signal != null) {
									final songStatus = signal.message.songStatus;

									WidgetsBinding.instance.addPostFrameCallback((_) {
										if (songStatus.tracks.isNotEmpty) {
											controller.songOptions(songStatus.songOptions);
											controller.setTracks(songStatus.tracks);
											toEditPage(controller);
										} else {
											AlertMessage.error("Invalid MIDI file");
										}
									});
								}

								return const Text("Drop a file here");
							},
						),
					]),
				)
			),
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
