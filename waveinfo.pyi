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
    Enum of wav file formats (codecs) as defined in [RFC2361](https://datatracker.ietf.org/doc/html/rfc2361) or Microsoft multimedia definitions.
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
    CS_IMAADPCM = 0x0039
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
    MVI_MVI2 = 0x0084
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
    ISIAUDIO_2 = 0x1401
    SOUNDSPACE_MUSICOMPRESS = 0x1500
    DVM = 0x2000
    DTS = 0x0008
    DRM = 0x0009
    WMAVOICE9 = 0x000A
    WMAVOICE10 = 0x000B
    HP_DYN_VOICE = 0x001A
    INTEL_G723_1 = 0x0043
    INTEL_G729 = 0x0044
    SHARP_G726 = 0x0045
    VOXWARE_SC3 = 0x007A
    VOXWARE_SC3_1 = 0x007B
    MULTITUDE_FT_SX20 = 0x008A
    INFOCOM_ITS_G721_ADPCM = 0x008B
    CONVEDIA_G729 = 0x008C
    CONGRUENCY = 0x008D
    MEDIASONIC_G723 = 0x0093
    MALDEN_PHONYTALK = 0x00A0
    RACAL_RECORDER_GSM = 0x00A1
    RACAL_RECORDER_G720_A = 0x00A2
    RACAL_RECORDER_G723_1 = 0x00A3
    RACAL_RECORDER_TETRA_ACELP = 0x00A4
    NEC_AAC = 0x00B0
    RAW_AAC1 = 0x00FF
    PROSODY_8KBPS = 0x0094
    PHILIPS_CELP = 0x0120
    PHILIPS_GRUNDIG = 0x0121
    SANYO_LD_ADPCM = 0x0125
    SIPROLAB_ACEPLNET = 0x0130
    SIPROLAB_ACELP4800 = 0x0131
    SIPROLAB_ACELP8V3 = 0x0132
    SIPROLAB_G729 = 0x0133
    SIPROLAB_G729A = 0x0134
    SIPROLAB_KELVIN = 0x0135
    VOICEAGE_AMR = 0x0136
    G726ADPCM = 0x0140
    DICTAPHONE_CELP68 = 0x0141
    DICTAPHONE_CELP54 = 0x0142
    QUALCOMM_PUREVOICE = 0x0150
    QUALCOMM_HALFRATE = 0x0151
    TUBGSM = 0x0155
    MSAUDIO1 = 0x0160
    WMAUDIO2 = 0x0161
    WMAUDIO3 = 0x0162
    WMAUDIO_LOSSLESS = 0x0163
    WMASPDIF = 0x0164
    UNISYS_NAP_ADPCM = 0x0170
    UNISYS_NAP_ULAW = 0x0171
    UNISYS_NAP_ALAW = 0x0172
    UNISYS_NAP_16K = 0x0173
    SYCOM_ACM_SYC008 = 0x0174
    SYCOM_ACM_SYC701_G726L = 0x0175
    SYCOM_ACM_SYC701_CELP54 = 0x0176
    SYCOM_ACM_SYC701_CELP68 = 0x0177
    KNOWLEDGE_ADVENTURE_ADPCM = 0x0178
    FRAUNHOFER_IIS_MPEG2_AAC = 0x0180
    DTS_DS = 0x0190
    UHER_ADPCM = 0x0210
    ULEAD_DV_AUDIO = 0x0215
    ULEAD_DV_AUDIO_1 = 0x0216
    ILINK_VC = 0x0230
    RAW_SPORT = 0x0240
    ESST_AC3 = 0x0241
    GENERIC_PASSTHRU = 0x0249
    IPI_HSX = 0x0250
    IPI_RPELP = 0x0251
    CS2 = 0x0260
    SONY_SCX = 0x0270
    SONY_SCY = 0x0271
    SONY_ATRAC3 = 0x0272
    SONY_SPC = 0x0273
    TELUM_AUDIO = 0x0280
    TELUM_IA_AUDIO = 0x0281
    NORCOM_VOICE_SYSTEMS_ADPCM = 0x0285
    MICRONAS = 0x0350
    MICRONAS_CELP833 = 0x0351
    INTEL_MUSIC_CODER = 0x0401
    INDEO_AUDIO = 0x0402
    QDESIGN_MUSIC = 0x0450
    ON2_VP7_AUDIO = 0x0500
    ON2_VP6_AUDIO = 0x0501
    TPC = 0x0681
    LIGHTWAVE_LOSSLESS = 0x08AE
    LH_CODEC_CELP = 0x1101
    LH_CODEC_SBC8 = 0x1102
    LH_CODEC_SBC12 = 0x1103
    LH_CODEC_SBC16 = 0x1104
    MPEG_ADTS_AAC = 0x1600
    MPEG_RAW_AAC = 0x1601
    MPEG_LOAS = 0x1602
    NOKIA_MPEG_ADTS_AAC = 0x1608
    NOKIA_MPEG_RAW_AAC = 0x1609
    VODAFONE_MPEG_ADTS_AAC = 0x160A
    VODAFONE_MPEG_RAW_AAC = 0x160B
    MPEG_HEAAC = 0x1610
    VOXWARE_RT24_SPEECH = 0x181C
    SONICFOUNDRY_LOSSLESS = 0x1971
    INNINGS_TELECOM_ADPCM = 0x1979
    LUCENT_SX8300P = 0x1C07
    LUCENT_SX5363S = 0x1C0C
    CUSEEME = 0x1F03
    NTCSOFT_ALF2CM_ACM = 0x1FC4
    DTS2 = 0x2001
    MAKEAVIS = 0x3313
    DIVIO_MPEG4_AAC = 0x4143
    NOKIA_ADAPTIVE_MULTIRATE = 0x4201
    DIVIO_G726 = 0x4243
    LEAD_SPEECH = 0x434C
    LEAD_VORBIS = 0x564C
    WAVPACK_AUDIO = 0x5756
    ALAC = 0x6C61
    OGG_VORBIS_MODE_1 = 0x674F
    OGG_VORBIS_MODE_2 = 0x6750
    OGG_VORBIS_MODE_3 = 0x6751
    OGG_VORBIS_MODE_1_PLUS = 0x676F
    OGG_VORBIS_MODE_2_PLUS = 0x6770
    OGG_VORBIS_MODE_3_PLUS = 0x6771
    THREECOM_NBX = 0x7000
    OPUS = 0x704F
    FAAD_AAC = 0x706D
    AMR_NB = 0x7361
    AMR_WB = 0x7362
    AMR_WP = 0x7363
    GSM_AMR_CBR = 0x7A21
    GSM_AMR_VBR_SID = 0x7A22
    COMVERSE_INFOSYS_G723_1 = 0xA100
    COMVERSE_INFOSYS_AVQSBC = 0xA101
    COMVERSE_INFOSYS_SBC = 0xA102
    SYMBOL_G729_A = 0xA103
    VOICEAGE_AMR_WB = 0xA104
    INGENIENT_G726 = 0xA105
    MPEG4_AAC = 0xA106
    ENCORE_G726 = 0xA107
    ZOLL_ASAO = 0xA108
    SPEEX_VOICE = 0xA109
    VIANIX_MASC = 0xA10A
    WM9_SPECTRUM_ANALYZER = 0xA10B
    WMF_SPECTRUM_ANAYZER = 0xA10C
    GSM_610 = 0xA10D
    GSM_620 = 0xA10E
    GSM_660 = 0xA10F
    GSM_690 = 0xA110
    GSM_ADAPTIVE_MULTIRATE_WB = 0xA111
    POLYCOM_G722 = 0xA112
    POLYCOM_G728 = 0xA113
    POLYCOM_G729_A = 0xA114
    POLYCOM_SIREN = 0xA115
    GLOBAL_IP_ILBC = 0xA116
    RADIOTIME_TIME_SHIFT_RADIO = 0xA117
    NICE_ACA = 0xA118
    NICE_ADPCM = 0xA119
    VOCORD_G721 = 0xA11A
    VOCORD_G726 = 0xA11B
    VOCORD_G722_1 = 0xA11C
    VOCORD_G728 = 0xA11D
    VOCORD_G729 = 0xA11E
    VOCORD_G729_A = 0xA11F
    VOCORD_G723_1 = 0xA120
    VOCORD_LBC = 0xA121
    NICE_G728 = 0xA122
    FRACE_TELECOM_G729 = 0xA123
    CODIAN = 0xA124
    FLAC = 0xF1AC
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
