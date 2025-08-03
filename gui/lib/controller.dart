import 'package:get/get.dart';
import 'package:midi_to_mml/src/bindings/bindings.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:midi_to_mml/extensions/track.dart';

class LogData {
  final DateTime time;
  final String message;

  const LogData(
    this.time,
    this.message,
  );
}

class AppController extends GetxController {
  final packageInfo =
      PackageInfo(appName: "", packageName: "", version: "", buildNumber: "")
          .obs;

  final autoBootVelocity = false.obs;
  final autoEqualizeNoteLength = false.obs;
  final velocityMin = 0.obs;
  final velocityMax = 15.obs;
  final minGapForChord = 0.obs;
  final smallestUnit = 64.obs;

  final tracks = <SignalMmlTrack>[].obs;
  final currentTrack = Rx<SignalMmlTrack?>(null);

  final fileName = "new_song".obs;

  final playbackStatus = SignalPlayStatus.stop.obs;
  final playingLength = 0.obs;

  final listLog = RxList<LogData>([]);
  final isLoading = false.obs;

  AppController() {
    getAppVersion();
  }

  SignalMmlSongOptions get songOptions {
    return SignalMmlSongOptions(
        autoBootVelocity: autoBootVelocity.value,
        autoEqualizeNoteLength: autoEqualizeNoteLength.value,
        minGapForChord: minGapForChord.value,
        velocityMin: velocityMin.value,
        velocityMax: velocityMax.value,
        smallestUnit: smallestUnit.value);
  }

  set songOptions(SignalMmlSongOptions val) {
    autoBootVelocity.value = val.autoBootVelocity;
    autoEqualizeNoteLength.value = val.autoEqualizeNoteLength;
    minGapForChord.value = val.minGapForChord;
    velocityMin.value = val.velocityMin;
    velocityMax.value = val.velocityMax;
    smallestUnit.value = val.smallestUnit;
  }

  void getAppVersion() async {
    packageInfo(await PackageInfo.fromPlatform());
  }

  void setTracks(List<SignalMmlTrack> listNewTrack) {
    tracks(listNewTrack);

    if (currentTrack() == null) {
      currentTrack(listNewTrack.first);
    } else if (currentTrack()!.index >= listNewTrack.length) {
      currentTrack(listNewTrack.last);
    } else {
      currentTrack(listNewTrack[currentTrack()!.index]);
    }

    tracks.refresh();
  }

  /// Export the final MML result
  String exportMML() {
    String result = "$fileName\n\n";
    result +=
        "------------------------------------------------------------------------------------\n";
    result +=
        "|     MIDI to MML - https://github.com/cuikho210/revelation-mobile-midi-to-mml     |\n";
    result +=
        "------------------------------------------------------------------------------------\n\n";
    result +=
        "Copy each track below to correspond to each track in the game\n\n";

    for (final track in tracks) {
      result += "${track.title}\n";
      result += "${track.instrument.name}\n\n";
      result += "${track.mml}\n\n";
    }

    return result;
  }
}
