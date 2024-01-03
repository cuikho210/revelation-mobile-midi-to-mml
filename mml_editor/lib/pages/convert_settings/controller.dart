import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mml_editor/native.dart';

class Controller extends GetxController {
	var refreshKey = const Key("Ahihi do ngoc!").obs;
	var isAutoSplit = false.obs;
	var tracks = <List<int>>[].obs;

	Controller(Uint8List bytes) {
		_generateTrackData(bytes);
	}

	void setIsAutoSplit(bool? value) => isAutoSplit.value = value ?? false;

	void mergeTracks(int to, int from, int value) {
		tracks[to].add(value);
		tracks[from].remove(value);
		_updateRefreshKey();
	}

	void _generateTrackData(Uint8List bytes) async {
		var trackLength = await api.getTrackLength(bytes: bytes);
		
		for (var i = 0; i < trackLength; i++) {
			var track = [i];
			tracks.add(track);
		}

		_updateRefreshKey();
	}

	void _updateRefreshKey() {
		refreshKey.value = Key(DateTime.now().toString());
	}
}
