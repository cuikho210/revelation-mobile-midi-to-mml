import 'package:flutter/material.dart';
import 'package:get/get.dart';

class AlertError {
	AlertError(String content) {
		if (Get.context != null) {
			ScaffoldMessenger.of(Get.context!).showSnackBar(SnackBar(
				content: Text(content),
				backgroundColor: Theme.of(Get.context!).colorScheme.error,
			));
		}	
	}
}
