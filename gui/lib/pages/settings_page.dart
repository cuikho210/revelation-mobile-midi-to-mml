import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/components/keymap_config.dart';
import 'package:midi_to_mml/components/soundfont_config.dart';
import 'package:midi_to_mml/controller.dart';
import 'package:remixicon/remixicon.dart';

class _TabData {
	final Icon icon;
	final Icon selectedIcon;
	final String label;

	const _TabData({
		required this.icon,
		required this.selectedIcon,
		required this.label,
	});
}

class SettingsPage extends GetView<AppController> {
	const SettingsPage({ super.key });

	@override
	Widget build(BuildContext context) {
		final List<_TabData> tabs = [
			const _TabData(
				icon: Icon(Remix.file_music_line),
				selectedIcon: Icon(Remix.file_music_line),
				label: "Soundfonts",
			),
			const _TabData(
				icon: Icon(Remix.command_line),
				selectedIcon: Icon(Remix.command_fill),
				label: "Keymap",
			),
		];

		final List<Widget> views = [
			const SoundfontConfig(),
			const KeymapConfig(),
		];

		final navigationIndex = 0.obs;

		return Scaffold(
			appBar: AppBar(
				title: const Text("Settings"),
			),
			body: Obx(() {
				const breakPoint = 777.0;
				final deviceWidth = MediaQuery.of(context).size.width;

				if (deviceWidth >= breakPoint) {
					return Row(
						children: [
							NavigationRail(
								selectedIndex: navigationIndex(),
								onDestinationSelected: (index) => navigationIndex(index),
								labelType: NavigationRailLabelType.all,
								elevation: .4,
								destinations: tabs.map((tab) => NavigationRailDestination(
									icon: tab.icon,
									selectedIcon: tab.selectedIcon,
									label: Text(tab.label),
								)).toList(),
							),
							Expanded(child: views[navigationIndex()]),
						],
					);
				} else {
					return Column(
						children: [
							Expanded(child: views[navigationIndex()]),
							NavigationBar(
								selectedIndex: navigationIndex(),
								onDestinationSelected: (index) => navigationIndex(index),
								destinations: tabs.map((tab) => NavigationDestination(
									icon: tab.icon,
									selectedIcon: tab.selectedIcon,
									label: tab.label,
								)).toList(),
							),
						],
					);
				}
			}),
		);
	}
}

