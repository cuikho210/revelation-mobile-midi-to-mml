import 'package:flutter/material.dart';
import 'package:get/get_navigation/src/root/get_material_app.dart';
import 'package:midi_to_mml/pages/home_page.dart';
import 'package:rinf/rinf.dart';
import './messages/generated.dart';
import 'package:midi_to_mml/command_signals.dart';

void main() async {
	await initializeRust(
		assignRustSignal,
	);

	runApp(GetMaterialApp(
		home: const HomePage(),
		debugShowCheckedModeBanner: false,
		theme: ThemeData.from(
			colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xffff907f)),
			useMaterial3: true,
		),
	));

	loadSoundfont();
}

void loadSoundfont() async {
	const path = 'assets/soundfonts/gm.sf2';
	// const path = '/home/cuikho210/Documents/soundfonts/FluidR3_GM.sf2';
	LoadSoundfont.fromPath(path);
}

