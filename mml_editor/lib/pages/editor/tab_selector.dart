import 'package:flutter/material.dart';
import 'package:flutter_remix_icon/flutter_remix_icon.dart';
import 'package:gap/gap.dart';
import 'package:get/get.dart';
import '././editor_controller.dart';

class TabSelector extends StatelessWidget {
	final List<Track> tracks;
	final int selectedTabIndex;
	final Function(int index) updateTabIndex;
	final Function(int index) togglePlayStatus;

	const TabSelector({
		super.key,
		required this.tracks,
		required this.selectedTabIndex,
		required this.updateTabIndex,
		required this.togglePlayStatus,
	});

	List<Widget> getTabButtons() {
		List<Widget> result = [];

		for (final track in tracks) {
			result.add(TabButton(
				onTap: () => updateTabIndex(track.index),
				onActionTap: () => togglePlayStatus(track.index),
				label: "Track ${track.index}",
				isActive: track.index == selectedTabIndex,
				isPlay: track.isPlay,
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
			height: 44,
			child: ListView(
				scrollDirection: Axis.horizontal,
				children: getTabButtons(),
			),
		);
	}
}

class TabButton extends StatelessWidget {
	final Function() onTap;
	final Function() onActionTap;
	final String label;
	final bool isActive;
	final bool isPlay;

	const TabButton({
		super.key,
		required this.onTap,
		required this.onActionTap,
		required this.label,
		required this.isActive,
		required this.isPlay,
	});

	Icon getPlayStatusIcon() {
		if (isPlay) {
			return const Icon(RemixIcon.volume_up_line);
		} else {
			return const Icon(RemixIcon.volume_mute_line);
		}
	}

	@override
	Widget build(context) {
		return InkWell(
			onTap: onTap,
			child: Container(
				padding: const EdgeInsets.only(left: 16),
				decoration: BoxDecoration(
					color: Theme.of(context).colorScheme.primary.withAlpha(isActive ? 64 : 0),
					border: const Border(right: BorderSide(
						color: Color(0x10000000),
						width: 1,
					)),
				),
				child: Row(children: [
					Text(label),
					const Gap(8),
					IconButton(
						onPressed: onActionTap,
						icon: getPlayStatusIcon(),
					),
				],),
			),
		);
	}
}
