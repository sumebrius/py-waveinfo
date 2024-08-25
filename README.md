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
>>> wav.detail.bit_depth
16
>>> wav.detail.sample_rate
44100
```

A `WavFile` may be initialised by passing any of:

- A string or path-like object pointing at the file to be read
- A file-like object (eg. `io.BytesIO`) containing the file
- A bytes object containing the raw bytes of the file

If a non-valid wave file is provided, a `WavLoadError` exception will be raised.

## Roadmap

- Provide methods to return the audio data, both as raw data and decoded PCM (where possible)
- Expose detail on playlists and cues
