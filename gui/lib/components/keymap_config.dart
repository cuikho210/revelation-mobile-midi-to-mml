import 'package:flutter/material.dart';
import 'package:get/get_state_manager/get_state_manager.dart';
import 'package:midi_to_mml/controller.dart';

class KeymapConfig extends GetView<AppController> {
	const KeymapConfig({ super.key });

	@override
	Widget build(BuildContext context) {

		return ListView(children: const [
			Padding(
				padding: EdgeInsets.all(16),
				child: Text("Keymap config"),
			),
		]);
	}
}
