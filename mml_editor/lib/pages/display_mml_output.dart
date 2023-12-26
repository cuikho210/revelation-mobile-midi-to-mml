import 'package:flutter/material.dart';
import 'package:flutter_remix_icon/flutter_remix_icon.dart';
import 'package:gap/gap.dart';
import 'package:flutter/services.dart';

class DisplayMMLOutputPage extends StatelessWidget {
	final String mml;

	const DisplayMMLOutputPage({
		super.key,
		required this.mml,
	});

	@override
	Widget build(context) {
		return Scaffold(
			appBar: AppBar(
				title: const Text("View Exported MML"),
				elevation: 1,
			),
			body: ListView(
				padding: const EdgeInsets.all(16),
				children: [TrackView(
					index: 0,
					content: mml,
				)],
			),
		);
	}
}

class TrackView extends StatelessWidget {
	final int index;
	final String content;

	const TrackView({
		super.key,
		required this.index,
		required this.content,
	});

	@override
	Widget build(context) {
		return Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
			Row(children: [
				const Icon(RemixIcon.play_list_line),
				const Gap(8),
				Text("Track $index", style: Theme.of(context).textTheme.headlineMedium),
				const Gap(32),
				ElevatedButton.icon(
					icon: const Icon(RemixIcon.clipboard_line),
					label: const Text("Copy"),
					onPressed: () => copyToClipboard(context),
				),
			]),
			const Gap(16),
			Text(content),
			const Gap(16),
		]);
	}

	copyToClipboard(BuildContext context) {
		Clipboard.setData(ClipboardData(text: content));

		ScaffoldMessenger.of(context).showSnackBar(
			SnackBar(
				content: const Text("Copied to clipboard!"),
				duration: const Duration(seconds: 2),
				behavior: SnackBarBehavior.floating,
				shape: RoundedRectangleBorder(
					borderRadius: BorderRadius.circular(8),
				),
			)
		);
	}
}
