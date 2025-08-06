import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/src/bindings/signals/signals.dart';
import 'package:remixicon/remixicon.dart';

class ToastSignal {
  static void listen() {
    ToastErrorSignal.rustSignalStream.listen((pack) {
      final msg = pack.message;
      toastError(msg.title, msg.content);
    });
  }

  static void toastError(String title, String content) {
    Get.snackbar(
      title,
      content,
      backgroundColor: Get.theme.colorScheme.error,
      colorText: Get.theme.colorScheme.onError,
      icon: const Icon(RemixIcons.error_warning_line),
      isDismissible: true,
      duration: const Duration(seconds: 15),
    );
  }
}
