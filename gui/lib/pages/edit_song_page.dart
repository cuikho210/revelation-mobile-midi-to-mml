import 'package:flutter/material.dart';
import 'package:gap/gap.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/components/song_options.dart';
import 'package:midi_to_mml/messages/rust_to_dart.pb.dart';
import 'package:remixicon/remixicon.dart';
import 'package:midi_to_mml/components/track.dart';
import 'package:midi_to_mml/controller.dart';

class EditSongPage extends GetView<AppController> {
	const EditSongPage({ super.key });

	@override
	Widget build(context) {
		listenUpdateMmlTracksStream();

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
		await for (final signal in SignalUpdateMmlTracks.rustSignalStream) {
			WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
				controller.setTracks(signal.message.tracks);
			});
		}
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

