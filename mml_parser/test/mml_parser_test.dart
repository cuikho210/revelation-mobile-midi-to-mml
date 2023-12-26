import 'package:mml_parser/mml_parser.dart';
import 'package:test/test.dart';

void main() {
	test('getDurationFromRawString', () {
		var utils = Utils();

		expect(utils.getDurationFromRawString("r2"), 32);
		expect(utils.getDurationFromRawString("r4&r16"), 16 + 4);
		expect(utils.getDurationFromRawString("r1&r16&r32"), 64 + 4 + 2);
	});
}
