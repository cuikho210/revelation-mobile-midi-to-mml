import 'package:flutter/material.dart';

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
			title: Text("Track $trackName - $trackNoteLength notes"),
			subtitle: Text(instrumentName),
			trailing: Wrap(spacing: 8, children: [
				ElevatedButton(
					onPressed: () => {},
					child: const Text("Split")
				),
				ElevatedButton(
					onPressed: () => {},
					child: const Text("Merge")
				),
			]),
		);
	}
}
