# waveinfo

`waveinfo` is a python package to read wave audio files and expose details of their contents.

## Usage

```pycon
>>> from waveinfo import WavFile

>>> wav = WavFile("path/to/file.wav")
>>> wav.detail.format
Format.PCM
>>> wav.detail.duration
datetime.timedelta(seconds=42)
>>> wav.detail.channels
1
>>> wav.details.bit_depth
16
>>> wav.detail.sample_rate
44100
```

A `WavFile` may be initialised by passing (currently) either a path-like pointing to the file to be read, or a bytes object containing the contents of a valid wav file.

If a non-valid wave file is provided, a `WavLoadError` exception will be raised.

## Roadmap

- Allow initialisation with a file object
- Expose additional, optional data about the file:
  - Channel mask
  - Playlist/Cue detail
  - Additional metadata
- Actual documentation
- Expand list of known codecs
- Provide methods to return the audio data, both as raw data and decoded PCM (where possible)
