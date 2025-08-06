import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/components/keymap/keymap_manager.dart';
import 'package:midi_to_mml/src/bindings/signals/signals.dart';

class ApplyKeymapDialog extends GetView<KeymapManager> {
  final int trackIndex;

  const ApplyKeymapDialog(this.trackIndex, {super.key});

  @override
  Widget build(BuildContext context) {
    return Dialog(
      child: SizedBox(
        height: 512,
        child: Obx(
          () => ListView.builder(
            itemCount: controller.keymaps.length,
            itemBuilder: (context, index) {
              final keymap = controller.keymaps[index];
              return ListTile(
                title: Text(keymap.name),
                subtitle: keymap.isDefault
                    ? const Text('Default')
                    : const Text('User-created'),
                trailing: ElevatedButton(
                  onPressed: () {
                    SignalApplyKeymap(
                            trackIndex: trackIndex, keymap: keymap.entries)
                        .sendSignalToRust();
                    Navigator.of(context).pop();
                  },
                  child: const Text('Apply'),
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}
