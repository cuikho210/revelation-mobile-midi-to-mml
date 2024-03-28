# Flutter GUI for Revelation Mobile MIDI to MML

## About
The lib is written in Rust. The interface part that written in Flutter.  
They communicate with each other through [`rinf`](https://rinf.cunarist.com).  


## Dev

### Requirement

+ [Rust](https://www.rust-lang.org/)
+ [Rinf](https://rinf.cunarist.com/)
+ [Flutter](https://flutter.dev/)

### Build

```sh
git clone --depth 1 --single-branch https://github.com/cuikho210/revelation-mobile-midi-to-mml
cd revelation-mobile-midi-to-mml/gui
rinf message
flutter build linux # Or apk, appbundle, windows, ...
```

### Messages
After update the code on the `messages` directory, run `rinf message` to generate the new code.  

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


