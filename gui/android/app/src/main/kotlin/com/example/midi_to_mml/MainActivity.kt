package com.example.midi_to_mml

import io.flutter.embedding.android.FlutterActivity

class MainActivity: FlutterActivity() {
	init {
		System.loadLibrary("hub")
    }
}
