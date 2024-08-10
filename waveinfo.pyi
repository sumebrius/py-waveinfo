from enum import Enum
from os import PathLike
from typing import Union

class WavFile:
    """
    A wav file.

    Args:
        file: Either a path to a file to read in, or the raw bytes of the file.
    """

    def __init__(self, file: Union[str, PathLike, bytes]) -> None: ...

class WavDetail:
    """
    Information about a wav file.
    """

class RawDetail:
    """
    Raw details about a wave file.
    """

class Format:
    """
    Enum of wav file formats (codecs)
    """

class WavLoadError(Exception):
    """
    Exception raised if a file is not able to be read as a valid wav file
    """
