from os import PathLike
from typing import BinaryIO, Optional, Union

class WavFile:
    """
    Representation of a wave file

    Parameters:
        file: A path to a file to read in, the file itself, or the raw bytes of the file.

    Raises:
        WavLoadError: If the file is unable to be parsed

    Attributes:
        detail: Details about the wave file
        info: Optional metadata embedded in the file
        raw_details: Details about the file directly extracted from it
    """

    def __init__(self, file: Union[str, PathLike, BinaryIO, bytes]) -> None: ...

    detail: WavDetail
    raw_details: RawDetail
    info: dict[str, str]

class WavDetail:
    """
    Details about the wav file audio

    Parameters:
        file: A path to a file to read in, the file itself, or the raw bytes of the file.

    Attributes:
        format: The format/codec of the audio
        duration: Duration of the audio
        channels: Number of audio channels
        bit_depth: The bit depth (amplitude resolution) of the audio
        sample_rate: The audio sample rate in Hz
        channel_positions: Ordered list of the speaker positions of each channel
    """

    def __init__(self, file: Union[str, PathLike, BinaryIO, bytes]) -> None: ...

    format: Format
    duration: float
    channels: int
    bit_depth: int
    sample_rate: int
    channel_positions: list[SpeakerPosition]

class RawDetail:
    """
    Raw details about a wave file.

    Parameters:
        file: A path to a file to read in, the file itself, or the raw bytes of the file.

    Attributes:
        format_tag: Format tag code
        channels: Number of channels
        sample_rate: Sample rate in Hz
        data_rate: Average bit rate
        block_size: Internal data alignment of audio
        sample_depth: Valid bits per sample
        channel_mask: Channel speaker poisition mask
        subformat: Subformat GUID
        total_samples: Total number of samples (per channel) in file. From fact chunk if present, otherwise calculated from data chunk length
    """

    def __init__(self, file: Union[str, PathLike, BinaryIO, bytes]) -> None: ...

    format_tag: int
    channels: int
    sample_rate: int
    data_rate: int
    block_size: int
    sample_depth: int
    channel_mask: Optional[int]
    subformat: Optional[str]
    total_samples: int

class Format:
    """
    Enum of wav file formats (codecs) as defined in [RFC2361](https://datatracker.ietf.org/doc/html/rfc2361)
    """

    UNKNOWN = 0x0000
    PCM = 0x0001
    ADPCM = 0x0002
    IEEE_FLOAT = 0x0003
    VSELP = 0x0004
    IBM_CVSD = 0x0005
    ALAW = 0x0006
    MULAW = 0x0007
    OKI_ADPCM = 0x0010
    DVI_ADPCM = 0x0011
    MEDIASPACE_ADPCM = 0x0012
    SIERRA_ADPCM = 0x0013
    G723_ADPCM = 0x0014
    DIGISTD = 0x0015
    DIGIFIX = 0x0016
    DIALOGIC_OKI_ADPCM = 0x0017
    MEDIAVISION_ADPCM = 0x0018
    CU_CODEC = 0x0019
    YAMAHA_ADPCM = 0x0020
    SONARC = 0x0021
    DSPGROUP_TRUESPEECH = 0x0022
    ECHOSC1 = 0x0023
    AUDIOFILE_AF36 = 0x0024
    APTX = 0x0025
    AUDIOFILE_AF10 = 0x0026
    PROSODY_1612 = 0x0027
    LRC = 0x0028
    DOLBY_AC2 = 0x0030
    GSM610 = 0x0031
    MSNAUDIO = 0x0032
    ANTEX_ADPCME = 0x0033
    CONTROL_RES_VQLPC = 0x0034
    DIGIREAL = 0x0035
    DIGIADPCM = 0x0036
    CONTROL_RES_CR10 = 0x0037
    NMS_VBXADPCM = 0x0038
    ROLAND_RDAC = 0x0039
    ECHOSC3 = 0x003A
    ROCKWELL_ADPCM = 0x003B
    ROCKWELL_DIGITALK = 0x003C
    XEBEC = 0x003D
    G721_ADPCM = 0x0040
    G728_CELP = 0x0041
    MSG723 = 0x0042
    MPEG = 0x0050
    RT24 = 0x0052
    PAC = 0x0053
    MPEGLAYER3 = 0x0055
    LUCENT_G723 = 0x0059
    CIRRUS = 0x0060
    ESPCM = 0x0061
    VOXWARE = 0x0062
    CANOPUS_ATRAC = 0x0063
    G726_ADPCM = 0x0064
    G722_ADPCM = 0x0065
    DSAT = 0x0066
    DSAT_DISPLAY = 0x0067
    VOXWARE_BYTE_ALIGNED = 0x0069
    VOXWARE_AC8 = 0x0070
    VOXWARE_AC10 = 0x0071
    VOXWARE_AC16 = 0x0072
    VOXWARE_AC20 = 0x0073
    VOXWARE_RT24 = 0x0074
    VOXWARE_RT29 = 0x0075
    VOXWARE_RT29HW = 0x0076
    VOXWARE_VR12 = 0x0077
    VOXWARE_VR18 = 0x0078
    VOXWARE_TQ40 = 0x0079
    SOFTSOUND = 0x0080
    VOXWARE_TQ60 = 0x0081
    MSRT24 = 0x0082
    G729A = 0x0083
    MVI_MV12 = 0x0084
    DF_G726 = 0x0085
    DF_GSM610 = 0x0086
    ISIAUDIO = 0x0088
    ONLIVE = 0x0089
    SBC24 = 0x0091
    DOLBY_AC3_SPDIF = 0x0092
    ZYXEL_ADPCM = 0x0097
    PHILIPS_LPCBB = 0x0098
    PACKED = 0x0099
    RHETOREX_ADPCM = 0x0100
    IRAT = 0x0101
    VIVO_G723 = 0x0111
    VIVO_SIREN = 0x0112
    DIGITAL_G723 = 0x0123
    CREATIVE_ADPCM = 0x0200
    CREATIVE_FASTSPEECH8 = 0x0202
    CREATIVE_FASTSPEECH10 = 0x0203
    QUARTERDECK = 0x0220
    FM_TOWNS_SND = 0x0300
    BTV_DIGITAL = 0x0400
    VME_VMPCM = 0x0680
    OLIGSM = 0x1000
    OLIADPCM = 0x1001
    OLICELP = 0x1002
    OLISBC = 0x1003
    OLIOPR = 0x1004
    LH_CODEC = 0x1100
    NORRIS = 0x1400
    ISIAUDIO_ATT = 0x1401
    SOUNDSPACE_MUSICOMPRESS = 0x1500
    DVM = 0x2000
    EXTENSIBLE = 0xFFFE

class SpeakerPosition:
    """
    Enum of speaker positions.
    """

    FRONT_LEFT = 0x00000001
    FRONT_RIGHT = 0x00000002
    FRONT_CENTER = 0x00000004
    LOW_FREQUENCY = 0x00000008
    BACK_LEFT = 0x00000010
    BACK_RIGHT = 0x00000020
    FRONT_LEFT_OF_CENTER = 0x00000040
    FRONT_RIGHT_OF_CENTER = 0x00000080
    BACK_CENTER = 0x00000100
    SIDE_LEFT = 0x00000200
    SIDE_RIGHT = 0x00000400
    TOP_CENTER = 0x00000800
    TOP_FRONT_LEFT = 0x00001000
    TOP_FRONT_CENTER = 0x00002000
    TOP_FRONT_RIGHT = 0x00004000
    TOP_BACK_LEFT = 0x00008000
    TOP_BACK_CENTER = 0x00010000
    TOP_BACK_RIGHT = 0x00020000
    RESERVED = 0xFFFFFFFF

class WavLoadError(Exception):
    """
    Exception raised if a file is not able to be read as a valid wav file
    """
