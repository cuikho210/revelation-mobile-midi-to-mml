import 'package:flutter/material.dart';
import 'package:gap/gap.dart';
import 'package:get/get_state_manager/get_state_manager.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:remixicon/remixicon.dart';

class SoundfontConfig extends GetView<AppController> {
	const SoundfontConfig({ super.key });

	@override
	Widget build(BuildContext context) {

		return Column(children: [
			Padding(
				padding: const EdgeInsets.all(8),
				child: Row(
					mainAxisAlignment: MainAxisAlignment.end,
					children: [
						ElevatedButton.icon(
							onPressed: () => (),
							label: const Text("Import"),
							icon: const Icon(Remix.import_line),
						),
						const Gap(8),
						ElevatedButton.icon(
							onPressed: () => (),
							label: const Text("Apply"),
							icon: const Icon(Remix.check_line),
						),
					],
				),
			),
			Expanded(child: ListView(
				padding: const EdgeInsets.all(16),
				children: [
					const Text("Currently"),
					const Gap(16),
					Text("Developing area", style: Theme.of(context).textTheme.titleLarge),
					Text("Khu vực đang phát triển", style: Theme.of(context).textTheme.titleLarge),
					const Gap(16),
					Text("Please wait for the next version!", style: Theme.of(context).textTheme.titleLarge),
					Text("Vui lòng chờ phiên bản sau nhé!", style: Theme.of(context).textTheme.titleLarge),
				],
			)),
			const Divider(),
			Expanded(child: ListView(
				padding: const EdgeInsets.all(16),
				children: const [
					Text("Available")
				],
			)),
		]);
	}
}
