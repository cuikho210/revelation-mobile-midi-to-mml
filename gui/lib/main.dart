import 'package:midi_to_mml/toast_signals.dart';
import 'package:rinf/rinf.dart';
import 'src/bindings/bindings.dart';
import 'package:flutter/material.dart';
import 'package:get/get_navigation/src/root/get_material_app.dart';
import 'package:midi_to_mml/pages/home_page.dart';
import 'package:midi_to_mml/command_signals.dart';

void main() async {
  await initializeRust(
    assignRustSignal,
  );

  ToastSignal.listen();

  runApp(GetMaterialApp(
    home: HomePage(),
    debugShowCheckedModeBanner: false,
    theme: ThemeData.from(
      colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xffff907f)),
      useMaterial3: true,
    ),
  ));

  loadSoundfont();
}

void loadSoundfont() async {
  const path = 'assets/soundfonts/a320-neo.sf2';
  LoadSoundfont.fromPath(path);
}
