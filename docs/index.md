# `waveinfo` - A wave file reader for python

`waveinfo` is a python package for parsing details about wave audio files.

## Usage

Usage is fairly straightfoward. Initialise a `WavFile` with any of:

- A string or [PathLike](https://docs.python.org/3/library/os.html#os.PathLike) pointing to the wave file in question.
- A binary [file object](https://docs.python.org/3/glossary.html#term-file-object) containing the file.
- A [bytes](https://docs.python.org/3/library/stdtypes.html#bytes) object containing the full contents of the file.

```pycon
>>> from waveinfo import WavFile
>>> wav = WavFile("path/to/file.wav")
>>> # or ...
>>> with open("path/to/file.wav", "rb") as fh:
...     wav = WavFile(fh)
```

The most interesting attributes of a `WavFile` are `detail` - which contains details of the wave file audio, and `info` - which is a dict of any metadata that may be embedded in the file.

```pycon
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
>>> wav.info
{'Software': 'Lavf61.1.100'}
```

If you're _only_ interested in the details, you can initialise a `WavDetail` directly in the same way:

```pycon
>>> from waveinfo import WavDetail
>>> details = WavDetail("path/to/file.wav")
>>> details.format
Format.PCM
>>> # etc...
```
