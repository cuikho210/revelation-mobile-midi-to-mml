import 'package:flutter/material.dart';
import 'package:gap/gap.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/command_signals.dart';
import 'package:midi_to_mml/components/highlighted_mml.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/extensions/track.dart';
import 'package:remixicon/remixicon.dart';
import 'package:flutter/services.dart';

class TrackContent extends GetView<AppController> {
  const TrackContent({super.key});

  @override
  Widget build(context) {
    final scrollController = ScrollController();

    return Expanded(
      child: Column(children: [
        const _TrackControls(),
        const Gap(8),
        Expanded(
          child: SelectionArea(
            child: ListView(
              controller: scrollController,
              padding: const EdgeInsets.all(16),
              children: [
                Obx(() => Text(
                      controller.currentTrack()?.title ?? '',
                      style: Theme.of(context).textTheme.titleMedium,
                    )),
                Obx(() => Text("MIDI: "
                    "${controller.currentTrack()?.instrument.instrumentId}. "
                    "${controller.currentTrack()?.instrument.name}")),
                Obx(() => Text("Channel: "
                    "${controller.currentTrack()?.instrument.midiChannel}")),
                const Gap(16),
                Obx(() {
                  final track = controller.currentTrack();

                  return HighlightedMml(
                    text: track?.mml ?? '',
                    trackIndex: track?.index ?? 0,
                    scrollController: scrollController,
                  );
                }),
              ],
            ),
          ),
        ),
      ]),
    );
  }
}

class _TrackControls extends GetView<AppController> {
  const _TrackControls();

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
  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.sizeOf(context).width;
    final isOverBreakpoint = screenWidth > 888;

    var direction = Axis.vertical;

    if (isOverBreakpoint) {
      direction = Axis.horizontal;
    }

    return Padding(
      padding: const EdgeInsets.fromLTRB(8, 8, 8, 0),
      child: Flex(
        direction: direction,
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Obx(() => Text(
                "Track ${controller.currentTrack()?.index}",
                style: Theme.of(context).textTheme.headlineSmall,
              )),
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
              TextButton.icon(
                onPressed: () => openEqualizeTracksDialog(context),
                icon: const Icon(Remix.equalizer_2_line),
                label: const Text("Equalize"),
              ),
              TextButton.icon(
                onPressed: () => openRenameTrackDialog(context),
                icon: const Icon(Remix.edit_line),
                label: const Text("Rename"),
              ),
              TextButton.icon(
                onPressed: () => Clipboard.setData(ClipboardData(
                        text: controller.currentTrack()?.mml ?? ''))
                    .then((_) {
                  Get.showSnackbar(
                    const GetSnackBar(
                      message: "Copied to clipboard!",
                      duration: Duration(seconds: 3),
                    ),
                  );
                }),
                icon: const Icon(Remix.file_copy_line),
                label: const Text("Copy"),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class _MergeTracksDialog extends GetView<AppController> {
  final int indexA;

  const _MergeTracksDialog(this.indexA);

  List<Widget> getTrackButtons(BuildContext context) {
    return controller
        .tracks()
        .where((track) => track.index != indexA)
        .map((track) => ListTile(
              title: Text(track.title),
              subtitle: Text(track.instrument.name),
              trailing: ElevatedButton(
                  child: const Text("Merge"),
                  onPressed: () {
                    MergeTracks(indexA, track.index);
                    Navigator.of(context).pop();
                  }),
            ))
        .toList();
  }

  @override
  Widget build(context) {
    return Dialog(
        child: SizedBox(
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
    return controller
        .tracks()
        .where((track) => track.index != indexA)
        .map((track) => ListTile(
              title: Text(track.title),
              subtitle: Text(track.instrument.name),
              trailing: ElevatedButton(
                  child: const Text("Equalize"),
                  onPressed: () {
                    EqualizeTracks(indexA, track.index);
                    Navigator.of(context).pop();
                  }),
            ))
        .toList();
  }

  @override
  Widget build(context) {
    return Dialog(
        child: SizedBox(
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

    return Dialog(
        child: SizedBox(
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
