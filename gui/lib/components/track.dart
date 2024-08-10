import 'package:flutter/material.dart';
import 'package:gap/gap.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/command_signals.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/messages/rust_to_dart.pb.dart';
import 'package:midi_to_mml/messages/types.pb.dart';
import 'package:midi_to_mml/extensions/track.dart';
import 'package:remixicon/remixicon.dart';
import 'package:flutter/services.dart';

class TrackContent extends GetView<AppController> {
	const TrackContent({ super.key });

	void splitTrack() {
		final track = controller.currentTrack();
		if (track == null) return;
		SplitTrack(track.index);
	}

	void openMergeTracksDialog(BuildContext context) {
		final track = controller.currentTrack();
		if (track == null) return;

		showDialog(
			context: context,
			builder: (context) => _MergeTracksDialog(track.index),
		);
	}

	void openEqualizeTracksDialog(BuildContext context) {
		final track = controller.currentTrack();
		if (track == null) return;

		showDialog(
			context: context,
			builder: (context) => _EqualizeTracksDialog(track.index),
		);
	}

	void openRenameTrackDialog(BuildContext context) {
		final track = controller.currentTrack();
		if (track == null) return;

		showDialog(
			context: context,
			builder: (context) => _RenameTrackDialog(track.index),
		);
	}

	@override
	Widget build(context) {
		final screenWidth = MediaQuery.sizeOf(context).width;
		final isOverBreakpoint = screenWidth > 420;

		return Expanded(
			child: Column(children: [
				Builder(builder: (context) {
					var direction = Axis.vertical;
					var padding = const EdgeInsets.all(0);
					var gap = const SizedBox(height: 8);

					if (isOverBreakpoint) {
						direction = Axis.horizontal;
						padding = const EdgeInsets.all(16);
						gap = const SizedBox();
					}

					return Padding(
						padding: padding,
						child: Flex(
							direction: direction,
							mainAxisAlignment: MainAxisAlignment.spaceBetween,
							children: [
								Obx(() => Text(
									"Track ${controller.currentTrack()?.index}",
									style: Theme.of(context).textTheme.headlineSmall,
								)),
								gap,
								Wrap(
									alignment: WrapAlignment.center,
									crossAxisAlignment: WrapCrossAlignment.center,
									children: [
										TextButton.icon(
											onPressed: splitTrack,
											icon: const Icon(Remix.git_branch_line),
											label: const Text("Split"),
										),
										TextButton.icon(
											onPressed: () => openMergeTracksDialog(context),
											icon: const Icon(Remix.git_pull_request_line),
											label: const Text("Merge"),
										),
										Obx(() {
											final track = controller.currentTrack();

											IconData icon = Remix.volume_up_line;
											String label = "Mute";

											if (track == null || track.isMuted) {
												icon = Remix.volume_mute_line;
												label = "Muted";
											}

											return TextButton.icon(
												onPressed: () {
													if (track == null) return;
													track.isMuted = !track.isMuted;
													controller.currentTrack.refresh();
												},
												icon: Icon(icon),
												label: Text(label),
											);
										}),
										MenuAnchor(
											builder: (context, controller, child) {
												return IconButton(
													onPressed: () {
														if (controller.isOpen) {
															controller.close();
														} else {
															controller.open();
														}
													},
													icon: const Icon(Remix.more_2_line),
												);
											},
											menuChildren: [
												MenuItemButton(
													onPressed: () => openEqualizeTracksDialog(context),
													leadingIcon: const Icon(Remix.equalizer_2_line),
													child: const Text("Equalize"),
												),
												MenuItemButton(
													onPressed: () => openRenameTrackDialog(context),
													leadingIcon: const Icon(Remix.edit_line),
													child: const Text("Rename"),
												),
												MenuItemButton(
													onPressed: () => Clipboard.setData(
														ClipboardData(
															text: controller.currentTrack()?.mml ?? '')
														).then((_) {
															ScaffoldMessenger.of(context).showSnackBar(
																const SnackBar(content: Text("Copied to clipboard!"))
															);
														}
													),
													leadingIcon: const Icon(Remix.file_copy_line),
													child: const Text("Copy to clipboard"),
												),
											],
										),
									],
								),
							],
						),
					);
				}),

				const Gap(8),

				Expanded(
					child: Builder(builder: (context) {
						const paddingValue = 16.0;
						var padding = const EdgeInsets.all(paddingValue);

						if (isOverBreakpoint) {
							padding = const EdgeInsets.fromLTRB(paddingValue, 0, paddingValue, paddingValue);
						}

						return Padding(
							padding: padding,
							child: SelectionArea(child:
								ListView(children: [
									Obx(() => Text(
										controller.currentTrack()?.title ?? '',
										style: Theme.of(context).textTheme.titleMedium,
									)),
									Obx(() => Text(controller.currentTrack()?.instrument.name ?? '')),
									const Gap(16),
									Obx(() {
										final track = controller.currentTrack();

										return _HighlightedText(
											track?.mml ?? '',
											track?.index ?? 0,
										);
									}),
								]),
							),
						);
					}),
				),
			]),
		);
	}
}

class _HighlightedText extends StatefulWidget {
	final String text;
	final int trackIndex;

	const _HighlightedText(this.text, this.trackIndex);

	@override
	createState() => _HighlightedTextState();
}

class _HighlightedTextState extends State<_HighlightedText> {
	int charLength = 0;
	int charIndex = 0;
	int charEnd = 0;

	void listenNoteOnEventStream() async {
		await for (final signal in SignalMmlNoteOn.rustSignalStream) {
			final signalTrackIndex = signal.message.trackId.toInt();
			final signalCharIndex = signal.message.charIndex.toInt();

			if (widget.trackIndex == signalTrackIndex) {
				setState(() {
					charLength = signal.message.charLength.toInt();
					charIndex = signalCharIndex;
					charEnd = charIndex + charLength + 1;
				});
			}
		}
	}

	@override
	build(context) {
		listenNoteOnEventStream();

		String textBefore = '';
		String textCurrent = '';
		String textAfter = '';

		try {
			textBefore = widget.text.substring(0, charIndex);
			textCurrent = widget.text.substring(charIndex, charEnd);
			textAfter = widget.text.substring(charEnd);
		} catch (e) {
			textAfter = widget.text;
		}

		return SizedBox(child: Text.rich(TextSpan(children: [
			TextSpan(
				text: textBefore,
				style: TextStyle(
					color: Theme.of(context).colorScheme.onSurface,
				),
			),
			TextSpan(
				text: textCurrent,
				style: TextStyle(
					color: Theme.of(context).colorScheme.onPrimary,
					backgroundColor: Theme.of(context).colorScheme.primary,
				),
			),
			TextSpan(
				text: textAfter,
				style: TextStyle(
					color: Theme.of(context).colorScheme.onSurface,
				),
			),
		])));
	}
}

class TrackTabButton extends GetView<AppController> {
	final SignalMmlTrack track;

	const TrackTabButton({
		super.key,
		required this.track,
	});

	@override
	Widget build(context) {

		return Column(children: [
			Obx(() => TextButton.icon(
				onPressed: () => controller.currentTrack(track),
				icon: Builder(builder: (context) {
					const icon = ImageIcon(AssetImage("assets/icon-instruments/piano.png"));

					if (track.isMuted) {
						return Badge(
							label: Icon(
								Remix.volume_mute_line,
								color: Theme.of(context).colorScheme.onPrimary,
								size: 12,
							),
							backgroundColor: Theme.of(context).colorScheme.error,
							offset: const Offset(4, -4),
							child: icon,
						);
					}

					return icon;
				}),
				label: Text("Track ${track.index}"),
				style: ButtonStyle(
					shape: const WidgetStatePropertyAll(
						RoundedRectangleBorder(
							borderRadius: BorderRadius.zero,
						),
					),
					backgroundColor: WidgetStatePropertyAll(
						(track.index == controller.currentTrack()?.index) ?
						Get.theme.colorScheme.primaryContainer :
						Colors.transparent
					),
				),
			)),
			const Gap(4),
			Text(
				"${track.mmlNoteLength} notes",
				style: Theme.of(context).textTheme.labelSmall,
			),
			Text(
				track.name,
				style: Theme.of(context).textTheme.labelSmall,
			),
		]);
	}
}

class _MergeTracksDialog extends GetView<AppController> {
	final int indexA;

	const _MergeTracksDialog(this.indexA);

	List<Widget> getTrackButtons(BuildContext context) {
		return controller.tracks()
			.where((track) => track.index != indexA)
			.map((track) => ListTile(
				title: Text(track.title),
				subtitle: Text(track.instrument.name),
				trailing: ElevatedButton(
					child: const Text("Merge"),
					onPressed: () {
						MergeTracks(indexA, track.index);
						Navigator.of(context).pop();
					}
				),
			)).toList();
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

class _EqualizeTracksDialog extends GetView<AppController> {
	final int indexA;

	const _EqualizeTracksDialog(this.indexA);

	List<Widget> getTrackButtons(BuildContext context) {
		return controller.tracks()
			.where((track) => track.index != indexA)
			.map((track) => ListTile(
				title: Text(track.title),
				subtitle: Text(track.instrument.name),
				trailing: ElevatedButton(
					child: const Text("Equalize"),
					onPressed: () {
						EqualizeTracks(indexA, track.index);
						Navigator.of(context).pop();
					}
				),
			)).toList();
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

class _RenameTrackDialog extends GetView<AppController> {
	final int index;

	const _RenameTrackDialog(this.index);

	@override
	Widget build(context) {
		final formKey = GlobalKey<FormState>();

		final textController = TextEditingController(
			text: controller.currentTrack()?.name,
		);

		return Dialog(child: SizedBox(
			height: 200,
			child: Padding(
				padding: const EdgeInsets.all(16),
				child: Form(
					key: formKey,
					child: Column(
						children: [
							Text(
								"Rename track $index",
								style: Theme.of(context).textTheme.titleLarge,
							),

							Expanded(
								child: Center(
									child: TextFormField(
										controller: textController,
										decoration: const InputDecoration(
											label: Text("New track name"),
										),
										validator: (value) {
											if (value == null || value.isEmpty) {
												return "Please enter some text";
											}
											
											return null;
										},
									),
								),
							),
							const Gap(16),

							ElevatedButton.icon(
								onPressed: () {
									if (formKey.currentState!.validate()) {
										RenameTrack(index, textController.text);
										Navigator.of(context).pop();
									}
								},
								label: const Text("Rename"),
								icon: const Icon(Remix.edit_line),
							),
						],
					),
				),
			),
		));
	}
}
