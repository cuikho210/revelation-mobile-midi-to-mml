<div align="center">
  <h1>Revelation mobile MIDI to MML</h1>

  <p>
    <a href="https://github.com/cuikho210/revelation-mobile-midi-to-mml/releases">Releases</a> - 
    <a href="https://github.com/cuikho210/revelation-mobile-midi-to-mml?tab=readme-ov-file#donate">Donate</a>
  </p>

  <a href="https://play.google.com/store/apps/details?id=com.mtlkms.revelation_mobile_midi_to_mml">
    <img src="https://raw.githubusercontent.com/pioug/google-play-badges/main/svg/en.svg" height="64" />
  </a>
  <a href="https://apps.microsoft.com/detail/9nwbrmhf4tlh">
    <img src="https://get.microsoft.com/images/en-us%20dark.svg" height="64"/>
  </a>
  <br /> <br />

  <p>A tool used to convert MIDI files into MML code used in Revelation Mobile</p>
</div>

## Why Choose This Tool?

+ Keep chords intact without splitting into multiple tracks
+ Fine-tune volume range for optimal sound balance
+ Automatically boost volume when needed
+ Easily split and merge tracks as desired
+ Integrated player with real-time highlighting

## MML Guide
### Note
Syntax: `<note_name>[duration]`  

**note_name**:  
Represents the musical notes: C, D, E, F, G, A, B.  
These correspond to the solfège syllables: Do, Re, Mi, Fa, Sol, La, Si.  

**duration**:  
Indicates the length of the note. Common durations include:  
1: Whole note (the longest)
2: Half note
4: Quarter note, and so on. Higher numbers represent shorter notes (e.g., 8 for an eighth note, 16 for a sixteenth note).  
...

Example: C4 (C quarter note), G1 (G whole note), A32 (A thirty-second note)  

### Chord
Notes can be combined to form a chord using the : symbol to connect them.  
Each note in the chord is played simultaneously.  

Syntax: `<note1>[duration]:<note2>[duration]:<note3>[duration]`  

For example, C16:E16:G16 creates a chord with the notes C, E, and G, all with a duration of a sixteenth note.  

### Rest
Rests work similarly to notes, but instead of a note_name, you use the letter r to indicate silence for a specific duration.  

Syntax: `r[duration]`  

For example, r4 represents a quarter rest, meaning a pause or silence for the length of a quarter note.  

### Tempo
Tempo sets the speed of the music, specifying the number of beats per minute (BPM).  

Syntax: `t<beats_per_minute>`  

For example, t120 sets the tempo to 120 BPM, meaning there are 120 beats in one minute.  

### Octave
The octave determines the pitch range of the notes, with higher numbers representing higher pitches.  
The range typically goes from 0 (lowest) to 8 (highest).  

Syntax: `o<octave_number>`  

For example, o4 sets the notes to the 4th octave, which is the middle range.  

### Velocity
Velocity controls the intensity or volume of the notes, with values ranging from 0 (softest) to 15 (loudest).  

Syntax: `v<velocity_value>`  

For example, v10 sets the note's velocity to 10, resulting in a moderately loud note.  

## Song options guide
### Auto Boot Velocity

Automatically increases the velocity to the highest level within the defined range.  
The boost is calculated from the current maximum velocity to the highest note velocity.  

### Auto Equalize Note Length

Automatically balances the number of notes between two tracks when performing a split action, ensuring even distribution.  

### Velocity Min and Max

By default, the velocity range is 0-15. The velocity min and velocity max define the minimum and maximum range within which notes are allowed.  

### Min Gap for Chord

In MML:
1. Each track is allowed to have only one note or chord played at any given time.  
2. The position of the subsequent note depends on the length of the preceding note.  

When overlapping notes in MIDI are converted to MML, two scenarios can occur:  
1. If the start point of two notes is less than or equal to the min gap for chord, these notes will be combined into a chord.  
2. If the start point of the following note minus the start point of the preceding note is greater than the min gap for chord, the preceding note will be shortened so that the position of the following note is accurate.  

The min gap for chord acts as a threshold condition, measured in the smallest unit.  

### Smallest Unit

The smallest unit in the process of converting MIDI to MML, by default, is a 1/64 note.  

## Donate

#### Paypal

[<img src="https://www.paypalobjects.com/paypal-ui/logos/svg/paypal-color.svg" height="64px" />)](https://paypal.me/cuikho210)

#### Momo

<img
  src="https://github.com/cuikho210/revelation-mobile-midi-to-mml/assets/86552587/889d0c3c-a214-4ebc-8db3-48cce0570b20"
  height="256px"
/>
