import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
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
  int _charIndex = 0;
  int _charEnd = 0;
  DateTime? _lastUserScrollTime;

  @override
  void initState() {
    super.initState();
    _listenNoteOnEventStream();
    widget.scrollController.addListener(_scrollListener);
  }

  @override
  void dispose() {
    widget.scrollController.removeListener(_scrollListener);
    super.dispose();
  }

  void _scrollListener() {
    if (!widget.scrollController.hasClients) return;
    final position = widget.scrollController.position;
    if (position.userScrollDirection != ScrollDirection.idle) {
      _lastUserScrollTime = DateTime.now();
    }
  }

  void _listenNoteOnEventStream() {
    SignalMmlNoteOn.rustSignalStream.listen((signal) {
      final signalTrackIndex = signal.message.trackIndex.toInt();
      final signalCharIndex = signal.message.charIndex.toInt();

      if (widget.trackIndex == signalTrackIndex && mounted) {
        setState(() {
          final charLength = signal.message.charLength.toInt();
          _charIndex = signalCharIndex;
          _charEnd = _charIndex + charLength;
        });

        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (mounted) {
            _scrollToHighlight(context);
          }
        });
      }
    });
  }

  void _scrollToHighlight(BuildContext context) {
    final now = DateTime.now();
    if (_lastUserScrollTime != null &&
        now.difference(_lastUserScrollTime!) < const Duration(seconds: 1)) {
      return;
    }

    final position = widget.scrollController.position;
    if (position.atEdge && position.pixels >= position.maxScrollExtent) {
      if (position.maxScrollExtent > 0) return;
    }

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
      TextPosition(offset: _charEnd),
      Rect.zero,
    );

    final targetScrollOffset = highlightedWordOffset.dy;
    final maxScroll = widget.scrollController.position.maxScrollExtent;

    widget.scrollController.animateTo(
      targetScrollOffset > maxScroll ? maxScroll : targetScrollOffset,
      duration: const Duration(milliseconds: 200),
      curve: Curves.linear,
    );
  }

  @override
  void didUpdateWidget(covariant HighlightedMml oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.scrollController != oldWidget.scrollController) {
      oldWidget.scrollController.removeListener(_scrollListener);
      widget.scrollController.addListener(_scrollListener);
    }
    if (widget.text != oldWidget.text) {
      setState(() {
        _charIndex = 0;
        _charEnd = 0;
      });
    }
  }

  @override
  build(context) {
    String textBefore = '';
    String textCurrent = '';
    String textAfter = '';

    try {
      textBefore = widget.text.substring(0, _charIndex);
      textCurrent = widget.text.substring(_charIndex, _charEnd);
      textAfter = widget.text.substring(_charEnd);
    } catch (e) {
      textAfter = widget.text;
    }

    return SizedBox(
      child: Text.rich(
        TextSpan(
          style: Theme.of(context).textTheme.bodyMedium,
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
