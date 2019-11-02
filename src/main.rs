use std::{
    fs::File,
    i16, io,
    io::{BufWriter, Write},
    path::Path,
};

use sample::signal::{ConstHz, ScaleAmp, Sine, Square};
use sample::{signal, Signal};

/// Chunk labels
const RIFF_LABEL: &[u8] = b"RIFF";
const FORMAT_LABEL: &[u8] = b"WAVE";
const DATA_LABEL: &[u8] = b"data";
const FMT_LABEL: &[u8] = b"fmt ";

/// metadata chunk is always 16 bytes
const FMT_CHUNK_SIZE: u32 = 16;
/// 8 bytes for 4-byte string label plus 4-byte subchunk size
const HEADER_SIZE: u32 = 8;
/// 1 means PCM
const FORMAT_TYPE: u16 = 1;
/// Standard sample rate: 44.1 KHz
const SAMPLE_RATE: u32 = 44_100;
/// Standard 16-bit sound resolution
const BITS_PER_SAMPLE: u16 = 16;
/// Mono (for now)
const NUM_CHANNELS: u16 = 1;
const NUM_INTERVALS: u32 = 12;

/// The frequencies of the 88 keys on a piano. Note that
/// octaves of 8 (starting at the bottom, A0, and every 12
/// semitones after) are exact since they are 2-1 ratios of
/// A4, which is defined at 440 Hz
#[allow(clippy::unreadable_literal)]
const _PIANO_KEY_FREQS: [f64; 88] = [
    27.5, // A0
    29.135235094880603,
    30.86770632850774,
    32.703195662574814,
    34.647828872108995,
    36.70809598967593,
    38.8908729652601,
    41.20344461410873,
    43.65352892912548,
    46.24930283895429,
    48.99942949771866,
    51.91308719749314,
    55.0, // A1
    58.270470189761205,
    61.73541265701548,
    65.40639132514963,
    69.29565774421799,
    73.41619197935186,
    77.7817459305202,
    82.40688922821747,
    87.30705785825096,
    92.49860567790859,
    97.99885899543732,
    103.82617439498628,
    110.0, // A2
    116.54094037952241,
    123.47082531403096,
    130.81278265029925,
    138.59131548843598,
    146.83238395870373,
    155.5634918610404,
    164.81377845643493,
    174.6141157165019,
    184.99721135581717,
    195.99771799087463,
    207.65234878997256,
    220.0, // A3
    233.08188075904482,
    246.94165062806192,
    261.6255653005985, // middle C
    277.18263097687196,
    293.66476791740746,
    311.1269837220808,
    329.62755691286986,
    349.2282314330038,
    369.99442271163434,
    391.99543598174927,
    415.3046975799451,
    440.0, // A4/A440
    466.1637615180899,
    493.8833012561241,
    523.2511306011974,
    554.3652619537443,
    587.3295358348153,
    622.253967444162,
    659.2551138257401,
    698.456462866008,
    739.988845423269,
    783.9908719634989,
    830.6093951598906,
    880.0, // A5
    932.3275230361799,
    987.7666025122483,
    1046.5022612023947,
    1108.7305239074885,
    1174.6590716696305,
    1244.507934888324,
    1318.5102276514801,
    1396.912925732016,
    1479.977690846538,
    1567.9817439269978,
    1661.2187903197812,
    1760.0, // A6
    1864.6550460723597,
    1975.5332050244965,
    2093.0045224047894,
    2217.461047814977,
    2349.318143339261,
    2489.015869776648,
    2637.0204553029603,
    2793.825851464032,
    2959.955381693076,
    3135.9634878539955,
    3322.4375806395624,
    3520.0, // A7
    3729.3100921447194,
    3951.066410048993,
    4186.009044809579,
];

// The 11-bit values used to generate piano key notes on an NES
const PIANO_KEYS_PERIODS: [u16; 88] = [
    0x0fe3, 0x0efe, 0x0e27, 0x0d5b, 0x0c9c, 0x0be6, 0x0b3b, 0x0a9a, 0x0a01, 0x0972, 0x08ea, 0x086a,
    0x07f1, 0x077f, 0x0713, 0x06ad, 0x064d, 0x05f3, 0x059d, 0x054c, 0x0500, 0x04b8, 0x0474, 0x0434,
    0x03f8, 0x03bf, 0x0389, 0x0356, 0x0326, 0x02f9, 0x02ce, 0x02a6, 0x0280, 0x025c, 0x023a, 0x021a,
    0x01fb, 0x01df, 0x01c4, 0x01ab, 0x0193, 0x017c, 0x0167, 0x0152, 0x013f, 0x012d, 0x011c, 0x010c,
    0x00fd, 0x00ef, 0x00e1, 0x00d5, 0x00c9, 0x00bd, 0x00b3, 0x00a9, 0x009f, 0x0096, 0x008e, 0x0086,
    0x007e, 0x0077, 0x0070, 0x006a, 0x0064, 0x005e, 0x0059, 0x0054, 0x004f, 0x004b, 0x0046, 0x0042,
    0x003f, 0x003b, 0x0038, 0x0034, 0x0031, 0x002f, 0x002c, 0x0029, 0x0027, 0x0025, 0x0023, 0x0021,
    0x001f, 0x001d, 0x001b, 0x001a,
];

/// WAV file format:
///
/// Offset      Num bytes   Field ID        Description
/// (SubChunk 0)
/// 0           4           ChunkID         "RIFF" in ASCII
/// 4           4           ChunkSize       36 + SubChunk2Size, or more precisely:
///                                         4 + (8 + SubChunk1Size) + (8 + SubChunk2Size)
///                                         This is the size of the rest of the file following this
///                                         number
/// 8           4           Format          "WAVE" in ASCII
/// (SubChunk 1)
/// 12          4           Subchunk1ID     "fmt " in ASCII
/// 16          4           Subchunk1Size   16 (for PCM) - the size of the rest of this subchunk
/// 20          2           AudioFormat     1 (for PCM)
/// 22          2           NumChannels     Mono = 1, Stereo = 2, etc.
/// 24          4           SampleRate      Usually 44100 - samples per second
/// 28          4           ByteRate        Bytes per second (SampleRate * BlockAlign)
/// 32          2           BlockAlign      Bytes per frame (BitsPerSample/8 * NumChannels)
/// 34          2           BitsPerSample   Usually 16
/// (SubChunk 2)
/// 36          4           Subchunk2ID      "data" in ASCII
/// 40          4           Subchunk2Size    Number of samples (i.e. SampleRate * length of the file
///                                          in seconds) * BlockAlign
/// 44          *           Data             The actual sound data.
fn write_wav_header<T: Write>(
    wav_output_file: &mut T,
    file_size: u32,
    bytes_per_frame: u16,
    data_chunk_size: u32,
) -> io::Result<()> {
    wav_output_file.write_all(RIFF_LABEL)?;
    wav_output_file.write_all(&file_size.to_le_bytes())?;
    wav_output_file.write_all(&FORMAT_LABEL)?;

    wav_output_file.write_all(FMT_LABEL)?;
    wav_output_file.write_all(&FMT_CHUNK_SIZE.to_le_bytes())?;
    wav_output_file.write_all(&FORMAT_TYPE.to_le_bytes())?;
    wav_output_file.write_all(&NUM_CHANNELS.to_le_bytes())?;
    wav_output_file.write_all(&SAMPLE_RATE.to_le_bytes())?;
    wav_output_file.write_all(&(SAMPLE_RATE * (bytes_per_frame as u32)).to_le_bytes())?;
    wav_output_file.write_all(&bytes_per_frame.to_le_bytes())?;
    wav_output_file.write_all(&BITS_PER_SAMPLE.to_le_bytes())?;

    wav_output_file.write_all(DATA_LABEL)?;
    wav_output_file.write_all(&data_chunk_size.to_le_bytes())?;

    Ok(())
}

fn _create_sine_wave(sample_rate: f64, hz: f64, amp: f64) -> ScaleAmp<Sine<ConstHz>> {
    signal::rate(sample_rate).const_hz(hz).sine().scale_amp(amp)
}

#[allow(dead_code)]
const NTSC_CLOCK_HZ: u32 = 1_789_773;

// TODO support duty cycle?
fn create_nes_square_wave(sound_bytes: [u8; 4]) -> ScaleAmp<Square<ConstHz>> {
    // Byte 0: DDLC VVVV    Duty (D), envelope loop / length counter halt (L), constant volume (C), volume/envelope (V)
    // Byte 1: EPPP NSSS    Sweep unit: enabled (E), period (P), negate (N), shift (S)
    // Byte 2: TTTT TTTT    Timer low (T)
    // Byte 3: LLLL LTTT    Length counter load (L), timer high (T)
    let volume = sound_bytes[0] & 0b1111;
    let _const_vol = sound_bytes[0] >> 4 & 0b1 != 0;
    let _length_counter_halt = sound_bytes[0] >> 5 & 0b1 != 0;
    let _duty_cycle = sound_bytes[0] >> 6 & 0b11;

    // TODO sweep unit at mem[1]
    let period_high_bits = ((sound_bytes[3] & 0b111).wrapping_shl(8)) as u16;
    let period_low_bits = sound_bytes[2] as u16;

    let period = period_high_bits | period_low_bits;
    let _length_counter_load = sound_bytes[3] >> 3 & 0b1_1111;

    let freq = NTSC_CLOCK_HZ / (16 * (period + 1)) as u32;

    let amp = i16::MAX as f64 * (volume as f64 / 15.0);

    create_square_wave(SAMPLE_RATE.into(), freq.into(), amp)
}

#[allow(dead_code)]
fn create_square_wave(sample_rate: f64, hz: f64, amp: f64) -> ScaleAmp<Square<ConstHz>> {
    signal::rate(sample_rate)
        .const_hz(hz)
        .square()
        .scale_amp(amp)
}

fn write_wav<T: Write>(duration_s: u32, key_num: usize, wav_output_file: &mut T) -> io::Result<()> {
    let bytes_per_frame: u16 = (NUM_CHANNELS * BITS_PER_SAMPLE) / 8;
    let num_samples: u32 = SAMPLE_RATE * duration_s;
    let data_chunk_size: u32 = num_samples * (bytes_per_frame as u32) * NUM_INTERVALS;
    let file_size: u32 = 4 + HEADER_SIZE + FMT_CHUNK_SIZE + HEADER_SIZE + data_chunk_size;

    write_wav_header(wav_output_file, file_size, bytes_per_frame, data_chunk_size)?;

    for num_half_steps in 1..=NUM_INTERVALS as usize {
        let base_freq = create_nes_square_wave([
            0b1011_0111,
            0b0,
            (PIANO_KEYS_PERIODS[key_num] & 0xFF) as u8,
            (PIANO_KEYS_PERIODS[key_num] >> 8 & 0x111) as u8,
        ]);

        let piano_half_steps_above = create_nes_square_wave([
            0b1011_0111,
            0b0,
            (PIANO_KEYS_PERIODS[key_num + num_half_steps] & 0xFF) as u8,
            (PIANO_KEYS_PERIODS[key_num + num_half_steps] >> 8 & 0x111) as u8,
        ]);

        let signal_iter = base_freq
            .add_amp(piano_half_steps_above)
            .take(num_samples as usize);

        for signal in signal_iter {
            wav_output_file.write_all(&(signal[0] as i16).to_le_bytes())?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // key 48 is A4, aka A440
    let path = Path::new("a440_intervals.wav");
    let mut wav_output_file = BufWriter::with_capacity(1 << 20, File::create(path)?);
    write_wav(1, 48, &mut wav_output_file)
}
