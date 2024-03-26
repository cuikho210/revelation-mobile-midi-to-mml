import 'package:get/get.dart';
import 'package:midi_to_mml/messages/types.pb.dart';
import 'package:package_info_plus/package_info_plus.dart';

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
	
	AppController() {
		getAppVersion();
	}

	void getAppVersion() async {
		packageInfo(await PackageInfo.fromPlatform());
	}
}
