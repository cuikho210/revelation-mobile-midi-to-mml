import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/pages/status_detail_page.dart';
import 'package:remixicon/remixicon.dart';

class StatusBar extends GetView<AppController> {
  const StatusBar({super.key});

  @override
  Widget build(BuildContext context) {
    const containerHeight = 32.0;

    return Container(
      height: containerHeight,
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.primaryContainer,
      ),
      child: Row(children: [
        IconButton(
          onPressed: () => Get.to(
            const StatusDetailPage(),
            transition: Transition.cupertino,
          ),
          icon: const Icon(Remix.expand_up_down_line),
          iconSize: containerHeight / 2,
          tooltip: 'Expand',
        ),
        Expanded(child: Obx(() {
          if (controller.listLog().isNotEmpty) {
            return Text(controller.listLog().last.message);
          } else {
            return const Text('');
          }
        })),
      ]),
    );
  }
}
