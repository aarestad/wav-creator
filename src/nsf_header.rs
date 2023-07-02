use nom::{
    bytes::complete::{tag, take},
    number::complete::{le_u16, le_u8},
    IResult,
};
use std::ffi::CString;

/// offset  # of bytes  Function
/// ----------------------------
/// $000    5   STRING  'N','E','S','M',$1A (denotes an NES sound format file)
/// $005    1   BYTE    Version number $01 (or $02 for NSF2)
/// $006    1   BYTE    Total songs   (1=1 song, 2=2 songs, etc)
/// $007    1   BYTE    Starting song (1=1st song, 2=2nd song, etc)
/// $008    2   WORD    (lo, hi) load address of data ($8000-FFFF)
/// $00A    2   WORD    (lo, hi) init address of data ($8000-FFFF)
/// $00C    2   WORD    (lo, hi) play address of data ($8000-FFFF)
/// $00E    32  STRING  The name of the song, null terminated
/// $02E    32  STRING  The artist, if known, null terminated
/// $04E    32  STRING  The copyright holder, null terminated
/// $06E    2   WORD    (lo, hi) Play speed, in 1/1000000th sec ticks, NTSC (see text)
/// $070    8   BYTE    Bankswitch init values (see text, and FDS section)
/// $078    2   WORD    (lo, hi) Play speed, in 1/1000000th sec ticks, PAL (see text)
/// $07A    1   BYTE    PAL/NTSC bits
///                 bit 0: if clear, this is an NTSC tune
///                 bit 0: if set, this is a PAL tune
///                 bit 1: if set, this is a dual PAL/NTSC tune
///                 bits 2-7: reserved, must be 0
/// $07B    1   BYTE    Extra Sound Chip Support
///                 bit 0: if set, this song uses VRC6 audio
///                 bit 1: if set, this song uses VRC7 audio
///                 bit 2: if set, this song uses FDS audio
///                 bit 3: if set, this song uses MMC5 audio
///                 bit 4: if set, this song uses Namco 163 audio
///                 bit 5: if set, this song uses Sunsoft 5B audio
///                 bit 6: if set, this song uses VT02+ audio
///                 bit 7: reserved, must be zero
/// $07C    1   BYTE    Reserved for NSF2
/// $07D    3   BYTE    24-bit length of contained program data.
///                 If 0, all data until end of file is part of the program.
///                 If used, can be used to provide NSF2 metadata
///                 in a backward compatible way.
/// $080    nnn ----    The music program/data follows

#[allow(dead_code)] // for now
#[derive(Debug)]
pub struct NsfHeader {
    version_num: u8,
    total_songs: u8,
    starting_song: u8,
    load_address: u16,
    init_address: u16,
    play_address: u16,
    song_name: CString,
    artist_name: CString,
    copyright_holder: CString,
    play_speed_ntsc: u16,
    bankswitch_init: [u8; 8],
    play_speed_pal: u16,
    pal_ntsc_bits: u8,
    sound_chip_support: u8,
    nsf2_reserved: u8,
    data_length: u32,
}

fn trim_trailing_nuls(input: &[u8]) -> &[u8] {
    let mut length = 0;

    for c in input {
        if *c != 0 {
            length += 1
        }
    }

    &input[..length]
}

pub fn parse_nsf(input: &[u8]) -> IResult<&[u8], NsfHeader> {
    let (input, _) = tag("NESM\x1a")(input)?;
    let (input, version_num) = le_u8(input)?;
    let (input, total_songs) = le_u8(input)?;
    let (input, starting_song) = le_u8(input)?;
    let (input, load_address) = le_u16(input)?;
    let (input, init_address) = le_u16(input)?;
    let (input, play_address) = le_u16(input)?;
    let (input, song_name) = take(32usize)(input)?;
    let (input, artist_name) = take(32usize)(input)?;
    let (input, copyright_holder) = take(32usize)(input)?;
    let (input, play_speed_ntsc) = le_u16(input)?;
    let (input, bankswitch_init) = take(8usize)(input)?;
    let (input, play_speed_pal) = le_u16(input)?;
    let (input, pal_ntsc_bits) = le_u8(input)?;
    let (input, sound_chip_support) = le_u8(input)?;
    let (input, nsf2_reserved) = le_u8(input)?;
    let (input, data_length) = take(3usize)(input)?;

    Ok((
        input,
        NsfHeader {
            version_num,
            total_songs,
            starting_song,
            load_address,
            init_address,
            play_address,
            song_name: CString::new(trim_trailing_nuls(song_name)).expect("bad song name"),
            artist_name: CString::new(trim_trailing_nuls(artist_name)).expect("bad artist name"),
            copyright_holder: CString::new(trim_trailing_nuls(copyright_holder))
                .expect("bad copyright holder"),
            play_speed_ntsc,
            bankswitch_init: bankswitch_init.try_into().unwrap(),
            play_speed_pal,
            pal_ntsc_bits,
            sound_chip_support,
            nsf2_reserved,
            data_length: u32::from_le_bytes([data_length[0], data_length[1], data_length[2], 0]),
        },
    ))
}
