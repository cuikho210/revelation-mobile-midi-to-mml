import 'package:get/get.dart';

class Controller extends GetxController {
	var isAutoSplit = false.obs;
	void setIsAutoSplit(bool? value) => isAutoSplit.value = value ?? false;
}
