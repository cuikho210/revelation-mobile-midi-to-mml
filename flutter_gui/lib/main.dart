import 'package:flutter/material.dart';
import 'package:flutter_gui/src/rust/frb_generated.dart';
import 'package:get/get.dart';
import 'package:flutter_gui/src/hub_page/hub_page.dart';

Future<void> main() async {
	await RustLib.init();
	runApp(const MyApp());
}

class MyApp extends StatelessWidget {
	const MyApp({ super.key });

	@override
	Widget build(context) {
		return GetMaterialApp(
			title: "MIDI to MML",
			theme: ThemeData(
				colorScheme: ColorScheme.fromSeed(
					seedColor: const Color(0xffff907f),
				),
				useMaterial3: true,
				fontFamily: "Mali",
			),
			debugShowCheckedModeBanner: false,
			home: const HubPage(),
		);
	}
}
