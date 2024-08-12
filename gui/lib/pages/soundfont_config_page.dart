import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:remixicon/remixicon.dart';

class SoundfontConfigPage extends GetView<AppController> {
	const SoundfontConfigPage({ super.key });

	@override
	Widget build(BuildContext context) {
		return Scaffold(
			appBar: AppBar(
				title: Text("Ahihi", style: Theme.of(context).textTheme.titleMedium),
			),
			body: const Column(children: [
				Text("Soundfont config page")
			]),
		);
	}
}

