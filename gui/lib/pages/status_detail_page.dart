import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:midi_to_mml/src/bindings/signals/signals.dart';

class StatusDetailPage extends GetView<AppController> {
  const StatusDetailPage({super.key});

  List<Widget> listLogToWidget() {
    return controller.listLog().map((log) {
      Color textColor;
      String logLevelText;

      switch (log.level) {
        case SignalLogLevel.trace:
          textColor = Colors.grey;
          logLevelText = "TRACE";
          break;
        case SignalLogLevel.debug:
          textColor = Colors.blueGrey;
          logLevelText = "DEBUG";
          break;
        case SignalLogLevel.info:
          textColor = Colors.blue;
          logLevelText = "INFO";
          break;
        case SignalLogLevel.warn:
          textColor = Colors.orange;
          logLevelText = "WARN";
          break;
        case SignalLogLevel.error:
          textColor = Colors.red;
          logLevelText = "ERROR";
          break;
      }

      return Text(
        "[${log.time.hour.toString().padLeft(2, '0')}:"
        "${log.time.minute.toString().padLeft(2, '0')}:"
        "${log.time.second.toString().padLeft(2, '0')}"
        "] [$logLevelText] ${log.message}",
        style: TextStyle(color: textColor),
      );
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
