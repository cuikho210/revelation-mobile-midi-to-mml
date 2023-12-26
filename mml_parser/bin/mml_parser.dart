import 'package:mml_parser/mml_parser.dart' as mml_parser;

void main(List<String> arguments) {
	if (arguments.isNotEmpty) {
		final mml = arguments.first;
		mml_parser.Parser().parse(mml);
	} else {
		print("You need to push the MML code as an argument");
	}
}
