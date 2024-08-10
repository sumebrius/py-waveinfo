from os import PathLike
from typing import Union

class WavFile:
    """
    A wav file.

    :param file: Either a path to a file to read in, or the raw bytes of the file.
    """

    def __init__(self, file: Union[str, PathLike, bytes]) -> None: ...
