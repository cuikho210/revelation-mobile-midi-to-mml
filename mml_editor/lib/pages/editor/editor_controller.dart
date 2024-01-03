import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

class Track {
	final int index;
	final String mml;
	bool isPlay;

	Track({
		required this.index,
		required this.mml,
		required this.isPlay,
	});
}

class EditorController extends GetxController {
	var midiBytes = Uint8List(0);
	var refreshKey = const Key("Ahihi do ngoc!").obs;
	var currentTabIndex = 0.obs;
	var tracks = <Track>[].obs;

	EditorController(Uint8List bytes, List<String> mmls) {
		tracks.value = _getTracks(mmls);
		midiBytes = bytes;
	}

	void updateCurrentTabIndex(index) => currentTabIndex.value = index;
	
	String getCurrentTrackContent() => tracks()[currentTabIndex()].mml;

	void toggleTrackPlayStatus(index) {
		tracks[index].isPlay = !tracks[index].isPlay;
		refreshKey.value = _getNewRefreshKey();
	}
	
	List<Track> _getTracks(List<String> mmls) {
		List<Track> result = [];

		mmls.asMap().forEach((index, mml) {
			result.add(Track(
				index: index,
				mml: mml,
				isPlay: true,
			));
		});

		return result;
	}

	Key _getNewRefreshKey() {
		return Key(DateTime.now().toString());
	}
}
