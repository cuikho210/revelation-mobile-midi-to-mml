import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/components/keymap/keymap_manager.dart';
import 'package:midi_to_mml/components/keymap/keymap_models.dart';

class KeymapDetailEditPage extends StatefulWidget {
  final Keymap keymap;

  const KeymapDetailEditPage({super.key, required this.keymap});

  @override
  State<KeymapDetailEditPage> createState() => _KeymapDetailEditPageState();
}

class _KeymapDetailEditPageState extends State<KeymapDetailEditPage> {
  final KeymapManager keymapManager = Get.find<KeymapManager>();
  late TextEditingController _keymapNameController;
  late Map<int, int> _editingEntries;
  final Map<int, TextEditingController> _midiKeyControllers = {};
  final Map<int, TextEditingController> _valueControllers = {};

  @override
  void initState() {
    super.initState();
    _keymapNameController = TextEditingController(text: widget.keymap.name);
    _editingEntries = Map.from(widget.keymap.entries);
    _initializeControllers();
  }

  void _initializeControllers() {
    _midiKeyControllers.clear();
    _valueControllers.clear();
    for (var entry in _editingEntries.entries) {
      _midiKeyControllers[entry.key] =
          TextEditingController(text: entry.key.toString());
      _valueControllers[entry.key] =
          TextEditingController(text: entry.value.toString());
    }
  }

  @override
  void dispose() {
    _keymapNameController.dispose();
    _midiKeyControllers.forEach((key, controller) => controller.dispose());
    _valueControllers.forEach((key, controller) => controller.dispose());
    super.dispose();
  }

  void _addEntry() {
    setState(() {
      int newMidiKey = 60; // Default MIDI key
      while (_editingEntries.containsKey(newMidiKey)) {
        newMidiKey++; // Find an unused MIDI key
      }
      _editingEntries[newMidiKey] = 60; // Default value
      _midiKeyControllers[newMidiKey] =
          TextEditingController(text: newMidiKey.toString());
      _valueControllers[newMidiKey] = TextEditingController(text: '60');
    });
  }

  void _removeEntry(int midiKey) {
    setState(() {
      _editingEntries.remove(midiKey);
      _midiKeyControllers.remove(midiKey)?.dispose();
      _valueControllers.remove(midiKey)?.dispose();
    });
  }

  void _saveChanges() {
    final Map<int, int> newEntries = {};
    bool hasError = false;

    for (var entry in _midiKeyControllers.entries) {
      final int originalMidiKey = entry.key;
      final TextEditingController midiKeyController = entry.value;
      final TextEditingController valueController =
          _valueControllers[originalMidiKey]!;

      final int? midiKey = int.tryParse(midiKeyController.text);
      final int? value = int.tryParse(valueController.text);

      if (midiKey != null && value != null) {
        if (newEntries.containsKey(midiKey)) {
          hasError = true;
          Get.snackbar(
            'Duplicate MIDI Key',
            'MIDI key $midiKey is used multiple times. Please ensure all MIDI keys are unique.',
            snackPosition: SnackPosition.BOTTOM,
            backgroundColor: Colors.red,
            colorText: Colors.white,
          );
          return; // Stop saving if duplicate found
        }
        newEntries[midiKey] = value;
      } else {
        hasError = true;
      }
    }

    if (hasError) {
      Get.snackbar(
        'Invalid Input',
        'Please ensure all MIDI keys and values are valid numbers.',
        snackPosition: SnackPosition.BOTTOM,
        backgroundColor: Colors.red,
        colorText: Colors.white,
      );
      return;
    }

    final updatedKeymap = Keymap(
      id: widget.keymap.id,
      name: _keymapNameController.text,
      isDefault: widget.keymap.isDefault,
      entries: newEntries,
    );
    keymapManager.saveKeymap(updatedKeymap);
    Get.back();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: TextField(
          controller: _keymapNameController,
          decoration: const InputDecoration(
            hintText: 'Keymap Name',
            border: InputBorder.none,
          ),
          style: Theme.of(context).appBarTheme.titleTextStyle,
        ),
        actions: [
          TextButton(onPressed: _saveChanges, child: const Text("Save")),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Expanded(
              child: ListView.builder(
                itemCount: _editingEntries.length,
                itemBuilder: (context, index) {
                  final midiKey = _editingEntries.keys.elementAt(index);

                  return Card(
                    margin: const EdgeInsets.symmetric(vertical: 8.0),
                    child: Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Row(
                        children: [
                          Expanded(
                            child: TextFormField(
                              controller: _midiKeyControllers[midiKey],
                              keyboardType: TextInputType.number,
                              decoration: InputDecoration(
                                labelText:
                                    'MIDI Key (${KeymapManager.getNoteName(int.tryParse(_midiKeyControllers[midiKey]?.text ?? '0') ?? 0)})',
                              ),
                              onChanged: (text) {
                                setState(() {
                                  // Force rebuild to update labelText
                                });
                              },
                            ),
                          ),
                          const SizedBox(width: 8.0),
                          Expanded(
                            child: TextFormField(
                              controller: _valueControllers[midiKey],
                              keyboardType: TextInputType.number,
                              decoration: InputDecoration(
                                labelText:
                                    'Value (${KeymapManager.getNoteName(int.tryParse(_valueControllers[midiKey]?.text ?? '0') ?? 0)})',
                              ),
                              onChanged: (text) {
                                setState(() {
                                  // Force rebuild to update labelText
                                });
                              },
                            ),
                          ),
                          IconButton(
                            icon: const Icon(Icons.remove_circle),
                            onPressed: () => _removeEntry(midiKey),
                          ),
                        ],
                      ),
                    ),
                  );
                },
              ),
            ),
            ElevatedButton(
              onPressed: _addEntry,
              child: const Text('Add Entry'),
            ),
          ],
        ),
      ),
    );
  }
}
