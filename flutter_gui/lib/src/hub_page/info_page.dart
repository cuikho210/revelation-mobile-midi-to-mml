import 'package:flutter/material.dart';
import 'package:get/get.dart';
import './controller.dart';

class InfoPage extends StatelessWidget {
	const InfoPage({ super.key });

	@override
	Widget build(BuildContext context) {
		final c = Get.put(Controller());

		return Center(child: Column(
			mainAxisAlignment: MainAxisAlignment.center,
			children: [
				Text("MIDI to MML", style: Theme.of(context).textTheme.headlineLarge),
				const SizedBox(height: 8),
				const Text("Author: cuikho210"),
				Obx(() => Text("Version: ${c.appVersion}")),
				const Text("https://github.com/cuikho210/midi-to-mml"),
			],
		));
	}
}
