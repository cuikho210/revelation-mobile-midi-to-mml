import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/src/bindings/signals/signals.dart';

class TrackTabButton extends GetView<AppController> {
  final SignalMmlTrack track;

  const TrackTabButton({
    super.key,
    required this.track,
  });

  String getInstrumentImage(SignalInstrument instrumentData) {
    Map<String, String> instruments = {
      '0-7': 'piano',
      '24-25': 'archtop_guitar',
      '26-31': 'electric_guitar',
      '32-39': 'bass_guitar',
      '40-47': 'violin',
      '75-75': 'nose_flute',
    };

    var instrumentName = 'piano';

    if (instrumentData.midiChannel == 9) {
      instrumentName = 'bass_drum';
    } else {
      for (var key in instruments.keys) {
        var bounds = key.split('-').map(int.parse).toList();
        if (instrumentData.instrumentId >= bounds[0] &&
            instrumentData.instrumentId <= bounds[1]) {
          instrumentName = instruments[key]!;
        }
      }
    }

    return "assets/icon-instruments/$instrumentName.png";
  }

  @override
  Widget build(context) {
    return Column(children: [
      Obx(() => TextButton.icon(
            onPressed: () => controller.currentTrack(track),
            icon: ImageIcon(AssetImage(getInstrumentImage(track.instrument))),
            label: Text("Track ${track.index}"),
            style: ButtonStyle(
              shape: const WidgetStatePropertyAll(
                RoundedRectangleBorder(
                  borderRadius: BorderRadius.zero,
                ),
              ),
              backgroundColor: WidgetStatePropertyAll(
                  (track.index == controller.currentTrack()?.index)
                      ? Get.theme.colorScheme.primaryContainer
                      : Colors.transparent),
            ),
          )),
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
