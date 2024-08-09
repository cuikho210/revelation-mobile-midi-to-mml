import 'package:get/get.dart';
import 'package:midi_to_mml/messages/types.pb.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:midi_to_mml/extensions/track.dart';

class AppController extends GetxController {
	final packageInfo = PackageInfo(
		appName: "",
		packageName: "",
		version: "",
		buildNumber: ""
	).obs;

	final songOptions = SignalMmlSongOptions().obs;
	final tracks = <SignalMmlTrack>[].obs;
	final currentTrack = Rx<SignalMmlTrack?>(null);
	final fileName = "new_song".obs;

	final highlightCharIndex = 0.obs;
	final highlightCharEnd = 0.obs;
	
	AppController() {
		getAppVersion();
	}

	void getAppVersion() async {
		packageInfo(await PackageInfo.fromPlatform());
	}

	void setTracks(List<SignalMmlTrack> listNewTrack) {
		tracks(listNewTrack);
		tracks.refresh();

		if (currentTrack() == null) {
			currentTrack(listNewTrack.first);
		} else if (currentTrack()!.index >= listNewTrack.length) {
			currentTrack(listNewTrack.last);
		} else {
			currentTrack(listNewTrack[currentTrack()!.index]);
		}
	}

	/// Export the final MML result
	String exportMML() {
		String result = "$fileName\n\n";
		result += "------------------------------------------------------------------------------------\n";
		result += "|     MIDI to MML - https://github.com/cuikho210/revelation-mobile-midi-to-mml     |\n";
		result += "------------------------------------------------------------------------------------\n\n";
		result += "Copy each track below to correspond to each track in the game\n\n";

		for (final track in tracks) {
			result += "${track.title}\n\n";
			result += "${track.mml}\n\n";
		}

		return result;
	}
}
