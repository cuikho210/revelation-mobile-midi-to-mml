import 'package:flutter/material.dart';
import 'package:get/get_state_manager/get_state_manager.dart';
import 'package:midi_to_mml/controller.dart';

class KeymapConfig extends GetView<AppController> {
	const KeymapConfig({ super.key });

	@override
	Widget build(BuildContext context) {

		return ListView(
			padding: const EdgeInsets.all(16),
			children: const [
				Text("Keymap config"),
			],
		);
	}
}
