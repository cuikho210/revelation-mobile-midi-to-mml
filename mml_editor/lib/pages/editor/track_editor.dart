import 'package:flutter/material.dart';
import 'package:mml_parser/mml_parser.dart';
import 'package:mml_editor/utils.dart' as utils;
import 'package:mml_editor/config.dart';

class TrackEditor extends StatelessWidget {
	final String rawString;

	const TrackEditor({
		super.key,
		required this.rawString,
	});

	@override
	Widget build(context) {
		

		return SingleChildScrollView(
			scrollDirection: Axis.vertical,
			child: ListView(
				scrollDirection: Axis.horizontal,
				children: const [],
			),
		);
	}

	parseEvents() {
		final events = Parser().parse(rawString);
		
		for (var event in events) {
			if (event is SetRest) {

			}
		}
	}
}

class RestEvent extends StatelessWidget {
	final int duration;

	const RestEvent({
		super.key,
		required this.duration,
	});

	@override
	Widget build(context) {
		final u = utils.Utils();

		return Container(
			width: u.getNoteWidthFromDuration(duration).toDouble(),
			height: defaultNoteHeight.toDouble(),
			decoration: BoxDecoration(
				borderRadius: BorderRadius.circular(8),
				border: Border.all(
					width: 1,
					color: Theme.of(context).colorScheme.primary.withAlpha(32),
				),
			),
		);
	}
}
