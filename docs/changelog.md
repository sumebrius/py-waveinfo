# Changelog

## 2025-10-23 - v0.5.0

- Update `pyo3` dependency to allow building for newer python versions.

## 2024-09-24 - v0.4.0

- Add many more recognised format tags

## 2024-09-07 - v0.3.2

### Bugfixes

- Account for fact chunk possibly missing in any file.

## 2024-09-05 - v0.3.1

### Bugfixes

- Correctly extract codec from files with an extensible fmt chunk and well-known subformat GUID
- Correctly set speaker positions if channel mask isn't set in an extensible fmt chunk

## 2024-08-25 - v0.3.0

### Features

- Improvements to type annotations and docstrings
- Add info attribute for embedded metadata
- Add channel speaker positions to detail

### Bugfixes:

- Account for possible padding bits when parsing file
- Fix sample length calculation for multi-channel PCM files.

## 2024-08-12 - v0.2.0

- Can now read files from a python file-like object
- Expanded formats to cover all codecs covered in [RFC 2361](https://datatracker.ietf.org/doc/html/rfc2361)
- Fixed format tag in raw_details to expose raw value
- Added license
- Extend package project details

## 2024-08-11 - v0.1.0

- Initial package release
