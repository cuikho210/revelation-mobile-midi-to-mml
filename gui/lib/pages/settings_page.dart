import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:midi_to_mml/components/keymap/keymap_manager.dart';
import 'package:midi_to_mml/pages/keymap_editing_page.dart';
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
  const SettingsPage({super.key});

  @override
  Widget build(BuildContext context) {
    const breakPoint = 777.0;
    final List<_TabData> tabs = [
      const _TabData(
        icon: Icon(Remix.command_line),
        selectedIcon: Icon(Remix.command_fill),
        label: "Keymap",
      ),
      const _TabData(
        icon: Icon(Remix.file_music_line),
        selectedIcon: Icon(Remix.file_music_line),
        label: "Soundfonts",
      ),
    ];

    final List<Widget> views = [
      KeymapEditingPage(),
      const SoundfontConfig(),
    ];

    final navigationIndex = 0.obs;

    return Scaffold(
      appBar: AppBar(
        title: const Text("Settings"),
      ),
      body: Obx(() {
        final deviceWidth = MediaQuery.of(context).size.width;

        if (deviceWidth >= breakPoint) {
          return Row(
            children: [
              NavigationRail(
                selectedIndex: navigationIndex(),
                onDestinationSelected: (index) => navigationIndex(index),
                labelType: NavigationRailLabelType.all,
                elevation: .4,
                destinations: tabs
                    .map((tab) => NavigationRailDestination(
                          icon: tab.icon,
                          selectedIcon: tab.selectedIcon,
                          label: Text(tab.label),
                        ))
                    .toList(),
              ),
              Expanded(child: views[navigationIndex()]),
            ],
          );
        } else {
          return Column(
            children: [
              Expanded(child: views[navigationIndex()]),
            ],
          );
        }
      }),
      bottomNavigationBar: Obx(() {
        if (MediaQuery.of(context).size.width < breakPoint) {
          return NavigationBar(
            selectedIndex: navigationIndex(),
            onDestinationSelected: (index) => navigationIndex(index),
            destinations: tabs
                .map((tab) => NavigationDestination(
                      icon: tab.icon,
                      selectedIcon: tab.selectedIcon,
                      label: tab.label,
                    ))
                .toList(),
          );
        }
        return const SizedBox.shrink(); // Hide NavigationBar on wider screens
      }),
      floatingActionButtonLocation: FloatingActionButtonLocation.endFloat,
      floatingActionButton: Obx(() {
        if (navigationIndex() == 0) {
          // Only show FAB on Keymap tab
          return FloatingActionButton(
            onPressed: () {
              final KeymapManager keymapManager = Get.find<KeymapManager>();
              keymapManager.createNewKeymap();
            },
            child: const Icon(Icons.add),
          );
        }
        return Container(); // Hide FAB on other tabs
      }),
    );
  }
}
