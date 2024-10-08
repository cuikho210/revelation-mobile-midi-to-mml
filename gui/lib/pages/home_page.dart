import 'package:desktop_drop/desktop_drop.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/command_signals.dart';
import 'package:midi_to_mml/components/status_bar.dart';
import 'package:midi_to_mml/messages/rust_to_dart.pb.dart';
import 'package:midi_to_mml/messages/types.pb.dart';
import 'package:midi_to_mml/pages/edit_song_page.dart';
import 'package:midi_to_mml/pages/settings_page.dart';
import 'package:midi_to_mml/utils.dart';
import 'package:remixicon/remixicon.dart';
import 'package:midi_to_mml/file_importer/from_midi_file.dart';
import 'package:gap/gap.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:url_launcher/url_launcher.dart';

class HomePage extends StatelessWidget {
	const HomePage({ super.key });

	toEditPage(AppController controller) async {
		await Get.to(
			const EditSongPage(),
			transition: Transition.cupertino,
		);

		StopSong();
		controller.playbackStatus(SignalPlayStatus.STOP);
	}

	listenLoadSongStream(AppController controller) {
		SignalLoadSongFromPathResponse.rustSignalStream.listen((signal) {
			final songStatus = signal.message.songStatus;

			if (songStatus.tracks.isNotEmpty) {
				controller.songOptions(songStatus.songOptions);
				controller.setTracks(songStatus.tracks);
				toEditPage(controller);
			} else {
				AlertMessage.error("Invalid MIDI file");
			}
		});
	}

	listenLogMessageStream(AppController controller) {
		SignalLogMessage.rustSignalStream.listen((signal) {
			final message = signal.message.message;
			final isLoading = signal.message.isLoading;

			controller.isLoading(isLoading);
			controller.listLog.add(LogData(
				DateTime.now(),
				message,
			));
			controller.listLog.refresh();
		});
	}

	void listenUpdateMmlTracksStream(AppController controller) async {
		SignalUpdateMmlTracks.rustSignalStream.listen((signal) {
			controller.setTracks(signal.message.tracks);
		});
	}

	void listenOnTrackEndStream(AppController controller) async {
		SignalOnTrackEnd.rustSignalStream.listen((signal) {
			if (controller.playbackStatus() == SignalPlayStatus.PLAY) {
				controller.playingLength(controller.playingLength() - 1);

				if (controller.playingLength() == 0) {
					controller.playbackStatus(SignalPlayStatus.STOP);
				}
			}
		});
	}

	@override
	Widget build(context) {
		final controller = Get.put(AppController());
		listenLoadSongStream(controller);
		listenLogMessageStream(controller);
		listenUpdateMmlTracksStream(controller);
		listenOnTrackEndStream(controller);

		return Scaffold(
			appBar: AppBar(
				title: const _AppTitle(),
				actions: [
					TextButton.icon(
						onPressed: () => Get.to(
							const SettingsPage(),
							transition: Transition.cupertino,
						),
						icon: const Icon(Remix.settings_line),
						label: const Text("Settings"),
					)
				],
			),
			body: DropTarget(
				onDragDone: (detail) {
					final path = detail.files.first.path;
					FromMidiFile.open(path);
				},
				child: Column(
					mainAxisAlignment: MainAxisAlignment.center,
					crossAxisAlignment: CrossAxisAlignment.center,
					children: [
						Expanded(child: Column(
							mainAxisAlignment: MainAxisAlignment.center,
							crossAxisAlignment: CrossAxisAlignment.center,
							children: [
								ElevatedButton.icon(
									onPressed: () => FromMidiFile.pickFile(),
									icon: const Icon(Remix.file_music_line),
									label: const Text("Import a MIDI file"),
								),

								const Gap(16),
								const Text("Drop a file here"),
								const Gap(24),

								Wrap(children: [
									TextButton.icon(
										icon: const Icon(Remix.github_line),
										label: const Text("Source code"),
										onPressed: () {
											final uri = Uri.parse("https://github.com/cuikho210/revelation-mobile-midi-to-mml");
											launchUrl(uri);
										},
									),

									TextButton.icon(
										icon: const Icon(Remix.hand_heart_line),
										label: const Text("Donate"),
										onPressed: () {
											final uri = Uri.parse("https://github.com/cuikho210/revelation-mobile-midi-to-mml?tab=readme-ov-file#donate");
											launchUrl(uri);
										},
									)
								])
							],
						)),
						const StatusBar(),
					],
				),
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
