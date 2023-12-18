import 'package:flutter/material.dart';
import 'package:flutter_remix_icon/flutter_remix_icon.dart';
import './home_page.dart';
import './projects_page.dart';
import './info_page.dart';

class HubPage extends StatefulWidget {
  const HubPage({ super.key });

  @override
  State<StatefulWidget> createState() => _HubPageState();
}

class _HubPageState extends State<HubPage> {
  int selectedIndex = 0;

  final pages = const [
    HomePage(),
    ProjectsPage(),
    InfoPage(),
  ];

  final railDestinations = const [
    NavigationRailDestination(
      icon: Icon(RemixIcon.home_2_line),
      selectedIcon: Icon(RemixIcon.home_2_fill),
      label: Text("Home"),
    ),
    NavigationRailDestination(
      icon: Icon(RemixIcon.projector_line),
      selectedIcon: Icon(RemixIcon.projector_fill),
      label: Text("Projects"),
    ),
    NavigationRailDestination(
      icon: Icon(RemixIcon.information_line),
      selectedIcon: Icon(RemixIcon.information_fill),
      label: Text("Info"),
    ),
  ];

  @override
  Widget build(BuildContext context) {
		return Scaffold(
			// appBar: AppBar(
			// 	title: const Text("MML Editor"),
      //   elevation: 1,
			// ),
			body: Row(children: [
        NavigationRail(
          destinations: railDestinations,
          selectedIndex: selectedIndex,
          extended: true,
          elevation: 1,
          onDestinationSelected: (value) => setState(() {
            selectedIndex = value;
          }),
        ),
        Expanded(child: pages[selectedIndex]),
      ]),
		);
  }
}