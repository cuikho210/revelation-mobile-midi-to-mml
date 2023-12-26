import 'package:mml_parser/mml_parser.dart';
import 'dart:io';

void main(List<String> arguments) async {
	final file = File("${Directory.current.path}/bin/test.mml");
	final mml = await file.readAsString();
	final events = Parser().parse(mml);
	
	for (var event in events) {
		if (event is SetNote) {
			print(event.rawString);
		}
	}
}
