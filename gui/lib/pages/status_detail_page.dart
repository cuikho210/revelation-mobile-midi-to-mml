import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/controller.dart';

class StatusDetailPage extends GetView<AppController> {
  const StatusDetailPage({super.key});

  List<Widget> listLogToWidget() {
    return controller.listLog().map((log) {
      return Text("[${log.time.hour}:"
          "${log.time.minute}:"
          "${log.time.second}"
          "] --> ${log.message}");
    }).toList();
  }

  @override
  Widget build(context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Log'),
      ),
      body: SelectionArea(
        child: ListView(
          padding: const EdgeInsets.all(16),
          children: listLogToWidget(),
        ),
      ),
    );
  }
}
