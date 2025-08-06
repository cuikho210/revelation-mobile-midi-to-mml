import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/command_signals.dart';
import 'package:remixicon/remixicon.dart';
import 'package:gap/gap.dart';
import 'package:url_launcher/url_launcher.dart';

class SongOptions extends GetView<AppController> {
  const SongOptions({super.key});

  @override
  Widget build(context) {
    final smallestUnitValues = [16, 32, 64, 128, 256];

    var smallestUnitIndex =
        smallestUnitValues.indexOf(controller.smallestUnit()).obs;

    return SizedBox(
      height: 512,
      child: Column(children: [
        Container(
            padding: const EdgeInsets.all(16),
            child: Flex(
              direction: Axis.horizontal,
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Row(children: [
                  const Icon(Remix.settings_line),
                  const Gap(8),
                  Text("Song Options",
                      style: Theme.of(context).textTheme.titleMedium),
                ]),
                ElevatedButton.icon(
                  label: const Text("Apply"),
                  icon: const Icon(Remix.save_line),
                  onPressed: () {
                    SaveSongOptions(controller.songOptions);
                    Navigator.pop(context);
                  },
                ),
              ],
            )),
        Expanded(
            child: ListView(children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: OutlinedButton.icon(
              onPressed: () {
                final uri = Uri.parse(
                    "https://github.com/cuikho210/revelation-mobile-midi-to-mml?tab=readme-ov-file#song-options-guide");
                launchUrl(uri);
              },
              label: const Text("Open Guide"),
              icon: const Icon(Remix.question_line),
            ),
          ),
          Obx(() => CheckboxListTile(
                title: const Text("Auto boot velocity"),
                value: controller.autoBootVelocity(),
                onChanged: (newValue) {
                  controller.autoBootVelocity(newValue == true);
                },
              )),
          Obx(() => CheckboxListTile(
                title: const Text("Auto equalize note length"),
                value: controller.autoEqualizeNoteLength(),
                onChanged: (newValue) {
                  controller.autoEqualizeNoteLength(newValue == true);
                },
              )),
          ListTile(
            title: const Text("Velocity min"),
            subtitle: Obx(() => Slider(
                value: controller.velocityMin.toDouble(),
                label: controller.velocityMin.toString(),
                min: 0,
                max: 15,
                divisions: 15,
                onChanged: (value) {
                  controller.velocityMin(value.toInt());
                })),
          ),
          ListTile(
            title: const Text("Velocity max"),
            subtitle: Obx(() => Slider(
                value: controller.velocityMax.toDouble(),
                label: controller.velocityMax.toString(),
                min: 0,
                max: 15,
                divisions: 15,
                onChanged: (value) {
                  controller.velocityMax(value.toInt());
                })),
          ),
          ListTile(
            title: const Text("Min gap for chord"),
            subtitle: Obx(() => Slider(
                value: controller.minGapForChord().toDouble(),
                label: controller.minGapForChord().toString(),
                min: 0,
                max: 16,
                divisions: 16,
                onChanged: (value) {
                  controller.minGapForChord(value.toInt());
                })),
          ),
          ListTile(
            title: const Text("Smallest unit"),
            subtitle: Obx(() => Slider(
                value: smallestUnitIndex().toDouble(),
                label: controller.smallestUnit.toString(),
                min: 0,
                max: smallestUnitValues.length - 1,
                divisions: smallestUnitValues.length - 1,
                onChanged: (newIndex) {
                  smallestUnitIndex(newIndex.toInt());
                  final value = smallestUnitValues[smallestUnitIndex()];
                  controller.smallestUnit(value);
                })),
          ),
        ])),
      ]),
    );
  }
}
