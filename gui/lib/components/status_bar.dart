import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/messages/rust_to_dart.pb.dart';

class StatusBar extends GetView<AppController> {
	const StatusBar({ super.key });

	listenLogMessageStream() {
		SignalLogMessage.rustSignalStream.listen((signal) {
			final message = signal.message;
			final isLoading = signal.message.isLoading;

			controller.isLoading(isLoading);
			controller.listLog.add(message.message);
			controller.listLog.refresh();
		});
	}
	
	@override
	Widget build(BuildContext context) {
		listenLogMessageStream();

		const containerHeight = 24.0;

		return Container(
			height: containerHeight,
			decoration: BoxDecoration(
				color: Theme.of(context).colorScheme.primaryContainer,
			),
			child: Row(children: [
				Expanded(child: Obx(() {
					if (controller.listLog().isNotEmpty) {
						return Text(controller.listLog().last);
					} else {
						return const Text('Ahihi');
					}
				})),
				SizedBox(
					width: containerHeight,
					height: containerHeight,
					child: Obx(() {
						if (controller.isLoading()) {
							return const CircularProgressIndicator(
								strokeWidth: 4,
							);
						} else {
							return const SizedBox();
						}
					}),
				),
			]),
		);
	}
}

