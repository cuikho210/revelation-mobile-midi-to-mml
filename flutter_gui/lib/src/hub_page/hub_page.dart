import "package:flutter/material.dart";
import "package:flutter_remix_icon/flutter_remix_icon.dart";
import "package:get/get.dart";
import "./controller.dart";
import "./home_page.dart";
import "./info_page.dart";

const railDestinations = <NavigationRailDestination>[
	NavigationRailDestination(
		icon: Icon(RemixIcon.home_2_line),
		selectedIcon: Icon(RemixIcon.home_2_fill),
		label: Text("Home"),
	),
	NavigationRailDestination(
		icon: Icon(RemixIcon.information_line),
		selectedIcon: Icon(RemixIcon.information_fill),
		label: Text("Info"),
	),
];

const pages = [
	HomePage(),
	InfoPage(),
];

class HubPage extends StatelessWidget {
	const HubPage({ super.key });

	@override
	Widget build(context) {
		final c = Get.put(Controller());

		return Scaffold(
			body: Row(children: [
				Obx(() => NavigationRail(
					destinations: railDestinations,
					selectedIndex: c.currentPageIndex(),
					onDestinationSelected: c.setCurrentPageIndex,
					extended: true,
					elevation: 1,
				)),
				Obx(() => Expanded(child: pages[c.currentPageIndex()])),
			]),
		);
	}
}
