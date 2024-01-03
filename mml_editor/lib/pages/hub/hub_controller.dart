import 'package:get/get.dart';

class HubController extends GetxController {
	var currentPageIndex = 0.obs;

	void setCurrentPageIndex(index) => currentPageIndex.value = index;
}
