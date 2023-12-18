import 'package:flutter/material.dart';
import 'package:package_info_plus/package_info_plus.dart';

class InfoPage extends StatefulWidget {
  const InfoPage({ super.key });

  @override
  State<StatefulWidget> createState() => _InfoPageState();
}

class _InfoPageState extends State<InfoPage> {
  String version = "Loading...";

  @override
  Widget build(BuildContext context) {
    PackageInfo.fromPlatform().then((value) => setState(() {
      version = value.version;
    }));

    return Center(child: Column(mainAxisAlignment: MainAxisAlignment.center, children: [
      Text("MML Editor", style: Theme.of(context).textTheme.headlineLarge),
      const SizedBox(height: 8),
      const Text("Author: cuikho210"),
      Text("Version: $version"),
      const Text("https://github.com/cuikho210/midi-to-mml"),
    ]));
  }
}