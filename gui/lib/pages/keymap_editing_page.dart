import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/components/keymap/keymap_manager.dart';
import 'package:midi_to_mml/pages/keymap_detail_edit_page.dart';

class KeymapEditingPage extends StatelessWidget {
  final KeymapManager keymapManager = Get.find<KeymapManager>();

  KeymapEditingPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Obx(
      () => ListView.builder(
        itemCount: keymapManager.keymaps.length,
        itemBuilder: (context, index) {
          final keymap = keymapManager.keymaps[index];
          return Card(
            margin: const EdgeInsets.all(8.0),
            child: ExpansionTile(
              title: Text(keymap.name),
              subtitle: keymap.isDefault
                  ? const Text('Default (Read-only)')
                  : const Text('User-created'),
              children: [
                Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text('ID: ${keymap.id}'),
                      const SizedBox(height: 8.0),
                      Text('Entries: ${keymap.entries.length}'),
                      const SizedBox(height: 16.0),
                      ...keymap.entries.entries.map((entry) => Text(
                          '${entry.key} => ${entry.value} (${KeymapManager.getNoteName(entry.key)} => ${KeymapManager.getNoteName(entry.value)})')),
                      const SizedBox(height: 16.0),
                      if (keymap.isDefault)
                        ElevatedButton(
                          onPressed: () {
                            keymapManager.cloneKeymap(keymap.id);
                          },
                          child: const Text('Clone Keymap'),
                        )
                      else
                        Row(
                          mainAxisAlignment: MainAxisAlignment.end,
                          children: [
                            ElevatedButton(
                              onPressed: () {
                                Get.to(
                                    () => KeymapDetailEditPage(keymap: keymap));
                              },
                              child: const Text('Edit Entries'),
                            ),
                            const SizedBox(width: 8.0),
                            ElevatedButton(
                              onPressed: () {
                                showDialog(
                                  context: context,
                                  builder: (BuildContext context) {
                                    return AlertDialog(
                                      title: const Text("Delete Keymap"),
                                      content: Text(
                                          "Are you sure you want to delete '${keymap.name}'?"),
                                      actions: <Widget>[
                                        TextButton(
                                          onPressed: () {
                                            Navigator.of(context)
                                                .pop(); // Dismiss dialog
                                          },
                                          child: const Text("Cancel"),
                                        ),
                                        ElevatedButton(
                                          onPressed: () {
                                            keymapManager
                                                .deleteKeymap(keymap.id);
                                            Navigator.of(context)
                                                .pop(); // Dismiss dialog
                                          },
                                          style: ElevatedButton.styleFrom(
                                              backgroundColor: Colors.red),
                                          child: const Text("Delete"),
                                        ),
                                      ],
                                    );
                                  },
                                );
                              },
                              style: ElevatedButton.styleFrom(
                                  backgroundColor: Colors.red),
                              child: const Text('Delete Keymap'),
                            ),
                          ],
                        ),
                    ],
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}
