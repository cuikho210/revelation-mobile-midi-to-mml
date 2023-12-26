import 'package:flutter/material.dart';
import './header.dart';
import './tab_selector.dart';
import '../display_mml_output.dart';

class EditorPage extends StatefulWidget {
	final List<String> mmls;

	const EditorPage({
		super.key,
		required this.mmls,
	});

	@override
	State createState() => EditorPageState();
}

class EditorPageState extends State<EditorPage> {
	int selectedTabIndex = 0;

	@override
	Widget build(context) {
		final trackLength = widget.mmls.length;

		return Scaffold(
			body: Column(children: [
				const EditorHeader(),
				TabSelector(
					selectedTabIndex: selectedTabIndex,
					trackLength: trackLength,
					updateTabIndex: (index) => setState(() => selectedTabIndex = index),
				),
				Expanded(child: ListView(
					padding: const EdgeInsets.all(16),
					children: [
						TrackView(
							index: selectedTabIndex,
							content: widget.mmls[selectedTabIndex],
						),
					],
				)),
			]),
		);
	}
}
