import 'package:flutter/material.dart';

class TabSelector extends StatelessWidget {
  final int selectedTabIndex;
  final int trackLength;
  final Function(int index) updateTabIndex;

  const TabSelector({
    super.key,
    required this.selectedTabIndex,
    required this.trackLength,
    required this.updateTabIndex,
  });

  List<TabButton> getTabButtons() {
    List<TabButton> result = [];

    for (var i = 0; i < trackLength; i++) {
      result.add(TabButton(
        onTap: () => updateTabIndex(i),
        label: "Track $i",
        isActive: i == selectedTabIndex
      ));
    }

    return result;
  }

  @override
  Widget build(context) {
    return Container(
      decoration: const BoxDecoration(border: Border(bottom: BorderSide(
        color: Color(0x10000000),
        width: 1,
      ))),
      child: Row(children: getTabButtons()),
    );
  }
}

class TabButton extends StatelessWidget {
  final Function() onTap;
  final String label;
  final bool isActive;

  const TabButton({
    super.key,
    required this.onTap,
    required this.label,
    required this.isActive,
  });

  @override
  Widget build(context) {
    return InkWell(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.all(4),
        decoration: BoxDecoration(
          color: Theme.of(context).colorScheme.primary.withAlpha(isActive ? 32 : 0),
          border: const Border(right: BorderSide(
            color: Color(0x10000000),
            width: 1,
          )),
        ),
        child: Text(label),
      ),
    );
  }
}