import 'package:flutter/material.dart';
import 'package:flutter_remix_icon/remixicon.dart';
import 'package:get/get.dart';
import 'package:mml_editor/pages/hub/hub_page.dart';

class EditorHeader extends StatelessWidget {
	final String fileName;

	const EditorHeader({
		super.key,
		required this.fileName,
	});

	@override
	Widget build(context) {
		return Container(
			decoration: const BoxDecoration(
				border: Border(bottom: BorderSide(
					color: Color(0x10000000),
					width: 1,
				)),
			),
			child: Row(children: [
				IconButton(
					icon: const Icon(RemixIcon.arrow_left_s_line),
					tooltip: "Back",
					onPressed: () => Get.offAll(
						const HubPage(),
						transition: Transition.leftToRight,
					),
				),
				IconButton(
					icon: const Icon(RemixIcon.settings_2_line),
					tooltip: "Options",
					onPressed: () => Get.back(),
				),
				Text(fileName),
				Expanded(child: Container()),
				IconButton(
					icon: const Icon(RemixIcon.play_line),
					tooltip: "Play",
					onPressed: () => (),
				),
				IconButton(
					icon: const Icon(RemixIcon.stop_line),
					tooltip: "Stop",
					onPressed: () => (),
				),
			],),
		);
	}
}

