import 'package:get/get.dart';
import 'package:get_storage/get_storage.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:logger/logger.dart';
import 'dart:convert';
import 'package:uuid/uuid.dart';

import 'keymap_models.dart';

class KeymapManager extends GetxController {
  final GetStorage _box = GetStorage();
  final _l = Logger();
  final _uuid = const Uuid();
  final RxList<Keymap> _keymaps = <Keymap>[].obs;

  List<Keymap> get keymaps => _keymaps.toList();

  @override
  void onInit() {
    super.onInit();
    _loadKeymaps();
  }

  Future<void> _loadKeymaps() async {
    // Load default keymaps from assets
    await _loadDefaultKeymaps();

    // Load user-created keymaps from GetStorage
    final List<dynamic>? storedKeymaps = _box.read('userKeymaps');
    if (storedKeymaps != null) {
      _keymaps
          .addAll(storedKeymaps.map((json) => Keymap.fromJson(json)).toList());
    }
  }

  Future<void> _loadDefaultKeymaps() async {
    try {
      final String response =
          await rootBundle.loadString('assets/keymap/default-percussion.json');
      final Map<String, dynamic> data = json.decode(response);

      Map<int, int> entries = {};
      data.forEach((key, value) {
        entries[int.parse(key)] = value;
      });

      final defaultKeymap = Keymap(
        id: 'default-percussion',
        name: 'Default Percussion',
        isDefault: true,
        entries: entries,
      );
      _keymaps.add(defaultKeymap);
    } catch (e) {
      _l.e(e);
    }
  }

  Keymap? getKeymapById(String id) {
    return _keymaps.firstWhereOrNull((km) => km.id == id);
  }

  void saveKeymap(Keymap keymap) {
    final int index = _keymaps.indexWhere((km) => km.id == keymap.id);
    if (index != -1) {
      _keymaps[index] = keymap;
    } else {
      _keymaps.add(keymap);
    }
    _saveUserKeymapsToStorage();
  }

  void deleteKeymap(String id) {
    _keymaps.removeWhere((km) => km.id == id && !km.isDefault);
    _saveUserKeymapsToStorage();
  }

  Keymap cloneKeymap(String id) {
    final Keymap? original = getKeymapById(id);
    if (original == null) {
      throw Exception('Keymap with id \$id not found for cloning.');
    }

    final newKeymap = Keymap(
      id: _uuid.v4(),
            name: "Copy of ${original.name}",
      isDefault: false,
      entries: Map.from(original.entries), // Deep copy the map
    );
    saveKeymap(newKeymap);
    return newKeymap;
  }

  void createNewKeymap() {
    final newKeymap = Keymap(
      id: _uuid.v4(),
      name: 'New Keymap',
      isDefault: false,
      entries: {},
    );
    saveKeymap(newKeymap);
  }

  void _saveUserKeymapsToStorage() {
    final List<Map<String, dynamic>> userKeymapsJson =
        _keymaps.where((km) => !km.isDefault).map((km) => km.toJson()).toList();
    _box.write('userKeymaps', userKeymapsJson);
  }

  // Utility to convert MIDI key to note name and octave
  static String getNoteName(int midiKey) {
    const noteNames = [
      'C',
      'C#',
      'D',
      'D#',
      'E',
      'F',
      'F#',
      'G',
      'G#',
      'A',
      'A#',
      'B'
    ];
    final noteIndex = midiKey % 12;
    final octave = (midiKey ~/ 12) - 1; // MIDI note 0 is C-1, so C4 is MIDI 60
    return '${noteNames[noteIndex]}$octave';
  }

  static int getMidiKey(String noteName, int octave) {
    const noteNames = [
      'C',
      'C#',
      'D',
      'D#',
      'E',
      'F',
      'F#',
      'G',
      'G#',
      'A',
      'A#',
      'B'
    ];
    final noteIndex = noteNames.indexOf(noteName);
    if (noteIndex == -1) {
      throw ArgumentError('Invalid note name: $noteName');
    }
    return (octave + 1) * 12 + noteIndex;
  }
}
