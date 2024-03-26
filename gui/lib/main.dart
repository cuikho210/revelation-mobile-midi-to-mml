import 'package:flutter/material.dart';
import 'package:get/get_navigation/src/root/get_material_app.dart';
import 'package:midi_to_mml/pages/home_page.dart';
import './messages/generated.dart';

void main() async {
	await initializeRust();

	runApp(GetMaterialApp(
		home: const HomePage(),
		debugShowCheckedModeBanner: false,
		theme: ThemeData.from(
			colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xffff907f)),
			useMaterial3: true,
		),
	));
}
