# Flutter GUI for Revelation Mobile MIDI to MML

## About

The lib is written in Rust. The interface part that written in Flutter.  
They communicate with each other through [`rinf`](https://rinf.cunarist.com).

## Dev

### Requirement

- [Rust](https://www.rust-lang.org/)
- [Rinf](https://rinf.cunarist.com/)
- [Flutter](https://flutter.dev/)

### Build

```sh
git clone --depth 1 --single-branch https://github.com/cuikho210/revelation-mobile-midi-to-mml
cd revelation-mobile-midi-to-mml/gui
rinf gen
flutter build linux # Or apk, appbundle, windows, ...
```

### Messages

After update the code on the `native/hub/src/signals` directory, run `rinf gen` to generate the new code.

### Dev on android

**Through capble**:

1. Plug the capble.
2. Enable USB debug mode on the phone.
3. Run `flutter run`.

**Wireless debugging**:

1. Enable Wireless debug in your phone
2. Click pair button on your phone
3. Run `adb pair <host>:[port]`
4. Enter the pair key
5. Run `adb connect <host>:[port]`
6. Run `flutter run`

## Using Rust Inside Flutter

This project leverages Flutter for GUI and Rust for the backend logic,
utilizing the capabilities of the
[Rinf](https://pub.dev/packages/rinf) framework.

To run and build this app, you need to have
[Flutter SDK](https://docs.flutter.dev/get-started/install)
and [Rust toolchain](https://www.rust-lang.org/tools/install)
installed on your system.
You can check that your system is ready with the commands below.
Note that all the Flutter subcomponents should be installed.

```bash
rustc --version
flutter doctor
```

You also need to have the CLI tool for Rinf ready.

```bash
cargo install rinf
```

Messages sent between Dart and Rust are implemented using Protobuf.
If you have newly cloned the project repository
or made changes to the `.proto` files in the `./messages` directory,
run the following command:

```bash
rinf gen
```

Now you can run and build this app just like any other Flutter projects.

```bash
flutter run
```

For detailed instructions on writing Rust and Flutter together,
please refer to Rinf's [documentation](https://rinf.cunarist.com).
