class KeymapEntry {
  int midiKey;
  int value;

  KeymapEntry({required this.midiKey, required this.value});

  factory KeymapEntry.fromJson(Map<String, dynamic> json) {
    return KeymapEntry(
      midiKey: json['midiKey'],
      value: json['value'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'midiKey': midiKey,
      'value': value,
    };
  }
}

class Keymap {
  String id;
  String name;
  bool isDefault;
  Map<int, int> entries;

  Keymap({required this.id, required this.name, required this.isDefault, required this.entries});

  factory Keymap.fromJson(Map<String, dynamic> json) {
    Map<int, int> entries = {};
    if (json['entries'] is Map) {
      (json['entries'] as Map).forEach((key, value) {
        entries[int.parse(key)] = value;
      });
    } else if (json['entries'] is List) {
      // Handle the case where entries might be a list of KeymapEntry objects (from previous incorrect model)
      for (var entryJson in json['entries']) {
        if (entryJson is Map && entryJson.containsKey('midiKey') && entryJson.containsKey('value')) {
          entries[entryJson['midiKey']] = entryJson['value'];
        }
      }
    }

    return Keymap(
      id: json['id'],
      name: json['name'],
      isDefault: json['isDefault'] ?? false,
      entries: entries,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'isDefault': isDefault,
      'entries': entries.map((key, value) => MapEntry(key.toString(), value)),
    };
  }
}