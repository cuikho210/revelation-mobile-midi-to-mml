import "package:get/get.dart";
import 'package:package_info_plus/package_info_plus.dart';

class Controller extends GetxController {
	var appVersion = "Loading...".obs;
	var currentPageIndex = 0.obs;

	Controller() {
		PackageInfo.fromPlatform().then((value) => appVersion.value = value.version);
	}

	void setCurrentPageIndex(index) => currentPageIndex.value = index;
}
