import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:gap/gap.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/command_signals.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/extensions/track.dart';
import 'package:midi_to_mml/messages/types.pb.dart';
import 'package:remixicon/remixicon.dart';

/// Display track title and MML content
class TrackAndMml extends StatelessWidget {
	final Track track;
	final String mml;

	const TrackAndMml({
		super.key,
		required this.track,
		required this.mml,
	});

	@override
	Widget build(context) {

		return Column(children: [
			ListTile(
				title: Text(track.title),
				subtitle: Text(track.instrumentName),
				trailing: ElevatedButton.icon(
					icon: const Icon(Remix.file_copy_line),
					label: const Text("Copy"),
					onPressed: () => Clipboard.setData(ClipboardData(text: mml)).then((_) {
						ScaffoldMessenger.of(context).showSnackBar(
							const SnackBar(content: Text("Copied to clipboard!"))
						);
					}),
				),
			),
			Container(
				height: 256,
				margin: const EdgeInsets.fromLTRB(16, 0, 64, 0),
				child: ListView(
					children: [
						Text(mml),
					],
				),
			),
			const Gap(16),
		]);
	}
}

class TrackListTitle extends StatelessWidget {
	final Track track;

	const TrackListTitle({
		super.key,
		required this.track,
	});

	@override
	Widget build(context) {
		return ListTile(
			title: Text(track.title),
			subtitle: Text(track.instrumentName),
			trailing: Wrap(spacing: 8, children: [
				ElevatedButton(
					child: const Text("Split"),
					onPressed: () => SplitTrack(track.index),
				),
				ElevatedButton(
					child: const Text("Merge"),
					onPressed: () => showDialog(
						context: context,
						builder: (context) => _MergeTracksDialog(track.index),
					),
				),
			]),
		);
	}
}

class _MergeTracksDialog extends GetView<AppController> {
	final int indexA;

	const _MergeTracksDialog(this.indexA);

	List<Widget> getTrackButtons(BuildContext context) {
		return controller.songStatus().tracks
			.where((track) => track.index != indexA)
			.map((track) => ListTile(
				title: Text(track.title),
				subtitle: Text(track.instrumentName),
				trailing: ElevatedButton(
					child: const Text("Merge"),
					onPressed: () {
						MergeTracks(indexA, track.index);
						Navigator.of(context).pop();
					}
				),
			))
			.toList();
	}

	@override
	Widget build(context) {
		return Dialog(child: SizedBox(
			height: 512,
			child: ListView(
				children: getTrackButtons(context),
			),
		));
	}
}
