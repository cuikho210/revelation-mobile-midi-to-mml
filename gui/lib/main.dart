import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/toast_signals.dart';
import 'package:rinf/rinf.dart';
import 'src/bindings/bindings.dart';
import 'package:flutter/material.dart';

import 'package:midi_to_mml/pages/home_page.dart';
import 'package:midi_to_mml/command_signals.dart';

import 'package:get/get.dart';
import 'package:get_storage/get_storage.dart';
import 'package:midi_to_mml/components/keymap/keymap_manager.dart';

void main() async {
  await GetStorage.init();
  await initializeRust(
    assignRustSignal,
  );

  ToastSignal.listen();
  Get.put(AppController());
  Get.put(KeymapManager()); // Initialize and put KeymapManager

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
