import 'package:flutter/material.dart';
import 'package:mml_editor/pages/hub/hub_page.dart';
import 'package:get/get.dart';

void main() {
	runApp(const MyApp());
}

class MyApp extends StatelessWidget {
	const MyApp({super.key});

	@override
	Widget build(BuildContext context) {
		return GetMaterialApp(
			title: 'MML Editor',
			theme: ThemeData(
				colorScheme: ColorScheme.fromSeed(
					seedColor: const Color(0xFFFF907F),
				),
				useMaterial3: true,
				fontFamily: "Mali",
			),
			home: const HubPage(),
			debugShowCheckedModeBanner: false,
		);
	}
}
