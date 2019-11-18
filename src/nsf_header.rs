use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct NsfHeader {
    magic: [u8; 5], // must be "NESM\x1a"
    version_num: u8,
    total_songs: u8,
    starting_song: u8,
    load_address: u16,
    init_address: u16,
    play_address: u16,
    song_name: [u8; 32],
    artist_name: [u8; 32],
    copyright_holder: [u8; 32],
    play_speed_ntsc: u16,
    bankswitch_init: [u8; 8],
    play_speed_pal: u16,
    pal_ntsc_bits: u8,
    sound_chip_support: u8,
    nsf2_reserved: u8,
    data_length: [u8; 3],
}