import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/command_signals.dart';
import 'package:midi_to_mml/controller.dart';

class TrackListTitle extends StatelessWidget {
	final int trackIndex;
	final String trackName;
	final String instrumentName;
	final int trackNoteLength;

	const TrackListTitle({
		super.key,
		required this.trackIndex,
		required this.trackName,
		required this.instrumentName,
		required this.trackNoteLength,
	});

	@override
	Widget build(context) {
		return ListTile(
			title: Text("$trackIndex. Track $trackName - $trackNoteLength notes"),
			subtitle: Text(instrumentName),
			trailing: Wrap(spacing: 8, children: [
				ElevatedButton(
					child: const Text("Split"),
					onPressed: () => SplitTrack(trackIndex),
				),
				ElevatedButton(
					child: const Text("Merge"),
					onPressed: () => showDialog(
						context: context,
						builder: (context) => _MergeTracksDialog(trackIndex),
					),
				),
			]),
		);
	}
}

class _MergeTracksDialog extends GetView<AppController> {
	final int indexA;

	const _MergeTracksDialog(this.indexA);

	List<Widget> getTrackButtons() {
		return controller.songStatus().tracks
			.where((track) => track.index != indexA)
			.map((track) => ListTile(
				title: Text("${track.index}. Track ${track.name} - ${track.noteLength} notes"),
				subtitle: Text(track.instrumentName),
				trailing: ElevatedButton(
					child: const Text("Merge"),
					onPressed: () => MergeTracks(indexA, track.index),
				),
			))
			.toList();
	}

	@override
	Widget build(context) {
		return Dialog(child: SizedBox(
			height: 512,
			child: ListView(
				children: getTrackButtons(),
			),
		));
	}
}
