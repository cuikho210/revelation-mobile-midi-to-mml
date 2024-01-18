import "dart:io";
import "dart:typed_data";
import "package:flutter/material.dart";
import "package:get/get.dart";

class Utils {
	const Utils();

	void alert(String title, String content) {
		Get.defaultDialog(
			title: title,
			content: Text(content),
		);
	}

	String getFileNameFromPath(String path) {
		String regStr = r"[^~)('!*<>:;,?";
		regStr += r'"*|\/\\]+\.mid';
		final regex = RegExp(regStr);

		final match = regex.firstMatch(path);

		if (match != null && match[0] != null) {
			final name = match[0]!;
			return name.substring(0, name.length - 4);
		} else {
			return "";
		}
	}

	Future<Uint8List> getBytesFromPath(String path) async {
		final file = File(path);
		final bytes = await file.readAsBytes();

		return bytes;
	}
}
