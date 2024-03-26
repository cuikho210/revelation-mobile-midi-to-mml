import 'package:flutter/material.dart';
import 'package:gap/gap.dart';
import 'package:midi_to_mml/messages/rust_to_dart.pb.dart';
import 'package:remixicon/remixicon.dart';
import 'package:midi_to_mml/components/track.dart';

class EditSongPage extends StatelessWidget {
	final SongStatus songStatus;

	const EditSongPage({
		super.key,
		required this.songStatus,
	});

	@override
	Widget build(context) {
		return Scaffold(
			appBar: AppBar(
				title: const Text("Edit"),
				actions: [
					TextButton.icon(
						icon: const Icon(Remix.export_line),
						label: const Text("Export"),
						onPressed: () => {},
					),
				],
			),
			body: ListView(children: [
				_Options(songStatus: songStatus),
				const Gap(32),
				_Tracks(songStatus: songStatus),
			]),
		);
	}
}

class _Tracks extends StatelessWidget {
	final SongStatus songStatus;

	const _Tracks({
		required this.songStatus,
	});

	List<Widget> getTrackWidgets() {
		return songStatus.tracks.map((track) => TrackListTitle(
			trackIndex: track.index,
			trackName: track.name,
			instrumentName: track.instrumentName,
			trackNoteLength: track.noteLength,
		)).toList();
	}

	@override
	Widget build(context) {
		return Column(children: [
			Text("Tracks", style: Theme.of(context).textTheme.titleLarge),
			const Gap(16),
			
			...getTrackWidgets()
		]);
	}
}

class _Options extends StatelessWidget {
	final SongStatus songStatus;

	const _Options({
		required this.songStatus,
	});

	@override
	Widget build(context) {
		return Column(children: [
			Text("Song options", style: Theme.of(context).textTheme.titleLarge),
			const Gap(16),

			CheckboxListTile(
				title: const Text("Auto boot velocity"),
				value: songStatus.options.autoBootVelocity,
				onChanged: (_) => {}
			),

			ListTile(
				title: const Text("Velocity min"),
				trailing: SizedBox(
					width: 48,
					child: TextFormField(
						textAlign: TextAlign.end,
						initialValue: songStatus.options.velocityMin.toString(),
						keyboardType: TextInputType.number,
					),
				),
			),

			ListTile(
				title: const Text("Velocity max"),
				trailing: SizedBox(
					width: 48,
					child: TextFormField(
						textAlign: TextAlign.end,
						initialValue: songStatus.options.velocityMax.toString(),
						keyboardType: TextInputType.number,
					),
				),
			),
		]);
	}
}
