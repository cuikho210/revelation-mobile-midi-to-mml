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

	final songStatus = SongStatus(
		options: SongOptions(
			autoBootVelocity: true,
			velocityMin: 0,
			velocityMax: 0,
		),
		tracks: [],
	).obs;

	final mmls = <String>[].obs;
	final fileName = "new_song".obs;
	
	AppController() {
		getAppVersion();
	}

	void getAppVersion() async {
		packageInfo(await PackageInfo.fromPlatform());
	}

	/// Export the final MML result
	String exportMML() {
		String result = "$fileName\n\n";
		result += "------------------------------------------------------------------------------------\n";
		result += "|     MIDI to MML - https://github.com/cuikho210/revelation-mobile-midi-to-mml     |\n";
		result += "------------------------------------------------------------------------------------\n\n";
		result += "Copy each track below to correspond to each track in the game\n\n";

		// for (int i = 0; i < mmls().length; i++) {
		// 	result += "${SongStatus().tracks[i].title}\n\n";
		// 	result += "${mmls()[i]}\n\n";
		// }

		for (final track in songStatus().tracks) {
			result += "${track.title}\n\n";
			result += "${mmls()[track.index]}\n\n";
		}

		return result;
	}
}
