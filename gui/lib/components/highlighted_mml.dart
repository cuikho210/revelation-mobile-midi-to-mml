import 'package:flutter/material.dart';
import 'package:midi_to_mml/src/bindings/signals/signals.dart';

class HighlightedMml extends StatefulWidget {
  final String text;
  final int trackIndex;
  final ScrollController scrollController;

  const HighlightedMml({
    super.key,
    required this.text,
    required this.trackIndex,
    required this.scrollController,
  });

  @override
  createState() => _HighlightedMmlState();
}

class _HighlightedMmlState extends State<HighlightedMml> {
  int charIndex = 0;
  int charEnd = 0;

  void listenNoteOnEventStream() async {
    SignalMmlNoteOn.rustSignalStream.listen((signal) {
      final signalTrackIndex = signal.message.trackIndex.toInt();
      final signalCharIndex = signal.message.charIndex.toInt();

      if (widget.trackIndex == signalTrackIndex && mounted) {
        setState(() {
          final charLength = signal.message.charLength.toInt();
          charIndex = signalCharIndex;
          charEnd = charIndex + charLength;
        });
      }
    });
  }

  void scrollToHighlight(BuildContext context) {
    final textSpan = TextSpan(
      text: widget.text,
      style: Theme.of(context).textTheme.bodyMedium,
    );

    final textPainter = TextPainter(
      text: textSpan,
      textAlign: TextAlign.left,
      textDirection: TextDirection.ltr,
    );

    textPainter.layout(
      minWidth: 0,
      maxWidth: MediaQuery.of(context).size.width - 32.0,
    );

    final highlightedWordOffset = textPainter.getOffsetForCaret(
      TextPosition(offset: charEnd),
      Rect.zero,
    );

    widget.scrollController.animateTo(
      highlightedWordOffset.dy,
      duration: const Duration(milliseconds: 200),
      curve: Curves.linear,
    );
  }

  @override
  void initState() {
    super.initState();
    listenNoteOnEventStream();
  }

  @override
  void didUpdateWidget(covariant HighlightedMml oldWidget) {
    super.didUpdateWidget(oldWidget);
    setState(() {
      charIndex = 0;
      charEnd = 0;
    });
  }

  @override
  build(context) {
    String textBefore = '';
    String textCurrent = '';
    String textAfter = '';

    try {
      textBefore = widget.text.substring(0, charIndex);
      textCurrent = widget.text.substring(charIndex, charEnd);
      textAfter = widget.text.substring(charEnd);
    } catch (e) {
      textAfter = widget.text;
    }

    if (textBefore.isNotEmpty) {
      scrollToHighlight(context);
    }

    return SizedBox(
      child: Text.rich(
        TextSpan(
          children: [
            TextSpan(text: textBefore),
            TextSpan(
              text: textCurrent,
              style: TextStyle(
                color: Theme.of(context).colorScheme.onPrimary,
                backgroundColor: Theme.of(context).colorScheme.primary,
              ),
            ),
            TextSpan(text: textAfter),
          ],
        ),
      ),
    );
  }
}
