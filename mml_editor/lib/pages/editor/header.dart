import 'package:flutter/material.dart';
import 'package:flutter_remix_icon/remixicon.dart';

class EditorHeader extends StatelessWidget {
  const EditorHeader({
    super.key,
  });

  @override
  Widget build(context) {
    return Container(
      decoration: const BoxDecoration(
        border: Border(bottom: BorderSide(
          color: Color(0x10000000),
          width: 1,
        )),
      ),
      child: Row(
        children: [
          IconButton(
            icon: const Icon(RemixIcon.arrow_left_s_line),
            tooltip: "Back",
            onPressed: () => Navigator.of(context).pop(),
          ),
          ElevatedButton.icon(
            onPressed: () => (),
            icon: const Icon(RemixIcon.save_line),
            label: const Text("Save"),
          ),
          Expanded(child: Container()),
          IconButton(
            icon: const Icon(RemixIcon.play_line),
            tooltip: "Play",
            onPressed: () => (),
          ),
          IconButton(
            icon: const Icon(RemixIcon.stop_line),
            tooltip: "Stop",
            onPressed: () => (),
          ),
        ],
      )
    );
  }
}