import 'package:flutter/material.dart';
import 'package:flutter_remix_icon/flutter_remix_icon.dart';
import 'package:get/get.dart';
import './home_page.dart';
import './projects_page.dart';
import './info_page.dart';
import '././hub_controller.dart';

class HubPage extends StatelessWidget {
	const HubPage({ super.key });

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
		final HubController controller = Get.put(HubController());

		return Scaffold(
			body: Row(children: [
				Obx(() => NavigationRail(
					destinations: railDestinations,
					selectedIndex: controller.currentPageIndex(),
					extended: true,
					elevation: 1,
					onDestinationSelected: (value) => controller.setCurrentPageIndex(value),
				)),
				Obx(() => Expanded(child: pages[controller.currentPageIndex()])),
			]),
		);
	}
}
