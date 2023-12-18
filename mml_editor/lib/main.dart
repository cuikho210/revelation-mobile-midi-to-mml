import 'package:flutter/material.dart';
import 'package:mml_editor/pages/hub/hub_page.dart';
import 'package:bitsdojo_window/bitsdojo_window.dart';

void main() {
	runApp(const MyApp());

	doWhenWindowReady(() {
		const initialSize = Size(800, 600);
		appWindow.minSize = initialSize;
		appWindow.size = initialSize;
		appWindow.show();
	});
}

class MyApp extends StatelessWidget {
	const MyApp({super.key});

	@override
	Widget build(BuildContext context) {
		return MaterialApp(
			title: 'MML Editor',
				theme: ThemeData(
				colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFFFF907F)),
				useMaterial3: true,
        fontFamily: "Mali",
			),
			home: const HubPage(),
			debugShowCheckedModeBanner: false,
		);
	}
}