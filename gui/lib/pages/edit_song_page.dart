import 'package:flutter/material.dart';
import 'package:gap/gap.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/command_signals.dart';
import 'package:midi_to_mml/components/song_options.dart';
import 'package:midi_to_mml/messages/rust_to_dart.pb.dart';
import 'package:midi_to_mml/messages/types.pb.dart';
import 'package:remixicon/remixicon.dart';
import 'package:midi_to_mml/components/track.dart';
import 'package:midi_to_mml/controller.dart';

class EditSongPage extends GetView<AppController> {
	const EditSongPage({ super.key });

	@override
	Widget build(context) {
		listenUpdateMmlTracksStream();
		listenOnTrackEndStream();

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
			body: const Column(children: [
				_SongControls(),
				Gap(8),
				_Tracks(),
				TrackContent(),
			]),
		);
	}

	void listenUpdateMmlTracksStream() async {
		SignalUpdateMmlTracks.rustSignalStream.listen((signal) {
			controller.setTracks(signal.message.tracks);
		});
	}

	void listenOnTrackEndStream() async {
		SignalOnTrackEnd.rustSignalStream.listen((signal) {
			controller.playingLength(controller.playingLength() - 1);

			if (controller.playingLength() == 0) {
				controller.playbackStatus(SignalPlayStatus.STOP);
			}
		});
	}
}

class _Tracks extends GetView<AppController> {
	const _Tracks();

	List<Widget> getListTrackTabButton() {
		return controller.tracks().map<Widget>((track) =>
			Padding(
				padding: const EdgeInsets.only(right: 4),
				child: TrackTabButton(track: track),
			)
		).toList();
	}

	@override
	Widget build(context) {
		final scrollController = ScrollController();

		return SizedBox(
			height: 72,
			child: Scrollbar(
				controller: scrollController,
				child: Obx(() => ListView(
					controller: scrollController,
					scrollDirection: Axis.horizontal,
					children: getListTrackTabButton(),
				)),
			),
		);
	}
}

class _SongControls extends GetView<AppController> {
	const _SongControls();

	Future<void> showOptionsModal(BuildContext context) async {
		return showModalBottomSheet<void>(context: context, builder: (context) {
			return const SongOptions();
		});
	}

	void playSong() {
		PlaySong();
		controller.playbackStatus(SignalPlayStatus.PLAY);
		controller.playingLength(controller.tracks().length);
	}

	void pauseSong() {
		PauseSong();
		controller.playbackStatus(SignalPlayStatus.PAUSE);
	}

	void stopSong() {
		if (controller.playbackStatus() != SignalPlayStatus.STOP) {
			StopSong();
			controller.playbackStatus(SignalPlayStatus.STOP);
		}
	}

	@override
	Widget build(context) {
		return Flex(
			direction: Axis.horizontal,
			mainAxisAlignment: MainAxisAlignment.spaceBetween,
			children: [
				TextButton.icon(
					onPressed: () => showOptionsModal(context),
					label: const Text("Song Options"),
					icon: const Icon(Remix.settings_line),
				),
				Row(children: [
					Obx(() {
						void Function() invoker = playSong;
						IconData icon = Remix.play_line;

						if (controller.playbackStatus() == SignalPlayStatus.PLAY) {
							invoker = pauseSong;
							icon = Remix.pause_line;
						}

						return IconButton(
							onPressed: invoker,
							icon: Icon(icon),
						);
					}),
					IconButton(
						onPressed: stopSong,
						icon: const Icon(Remix.stop_line),
					),
				])
			],
		);
	}
}

