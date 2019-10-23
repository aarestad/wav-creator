use std::{fs::File, i16, io, io::{Write, BufWriter}, path::Path};

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

///
/// Write a WAV file to `file_name`
///
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
///
/// This function always writes the WAV with the following parameters:
/// Sample rate: 44,100 samples/sec
/// Bits per sample: 16
/// Channels: 1 (monoaural)
/// The sound produced is a 12*`duration_s` second-long tune, consisting
/// of twelve `duration_s`-long intervals, starting at a half step going up to an octave, each
/// with a `freq` frequency sine wave as the base.
fn write_wav(duration_s: u32, freq: f64, amp: i16, file_name: &Path) -> io::Result<()> {
    let bytes_per_frame: u16 = (NUM_CHANNELS * BITS_PER_SAMPLE) / 8;
    let num_samples: u32 = SAMPLE_RATE * duration_s;
    let data_chunk_size: u32 = num_samples * (bytes_per_frame as u32) * NUM_INTERVALS;
    let file_size: u32 = 4 + HEADER_SIZE + FMT_CHUNK_SIZE + HEADER_SIZE + data_chunk_size;

    let mut wav_output_file = BufWriter::with_capacity(2 << 20, File::create(file_name)?);

    wav_output_file.write(RIFF_LABEL)?;
    wav_output_file.write(&file_size.to_le_bytes())?;
    wav_output_file.write(&FORMAT_LABEL)?;

    wav_output_file.write(FMT_LABEL)?;
    wav_output_file.write(&FMT_CHUNK_SIZE.to_le_bytes())?;
    wav_output_file.write(&FORMAT_TYPE.to_le_bytes())?;
    wav_output_file.write(&NUM_CHANNELS.to_le_bytes())?;
    wav_output_file.write(&SAMPLE_RATE.to_le_bytes())?;
    wav_output_file.write(&(SAMPLE_RATE * (bytes_per_frame as u32)).to_le_bytes())?;
    wav_output_file.write(&bytes_per_frame.to_le_bytes())?;
    wav_output_file.write(&BITS_PER_SAMPLE.to_le_bytes())?;

    wav_output_file.write(DATA_LABEL)?;
    wav_output_file.write(&data_chunk_size.to_le_bytes())?;

    let twelfth_root_of_two = 2.0f64.powf(1.0 / 12.0);

    for num_half_steps in 1..=NUM_INTERVALS {
        let base_freq = signal::rate(SAMPLE_RATE.into())
            .const_hz(freq)
            .sine()
            .scale_amp((amp / 2).into());

        let freq_multiple = twelfth_root_of_two.powi(num_half_steps as i32);

        let piano_half_steps_above = signal::rate(SAMPLE_RATE.into())
            .const_hz(freq * freq_multiple)
            .sine()
            .scale_amp((amp / 2).into());

        let signal_iter = base_freq
            .add_amp(piano_half_steps_above)
            .take(num_samples as usize);

        for signal in signal_iter {
            wav_output_file.write(&(signal[0] as i16).to_le_bytes())?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    write_wav(1, 440.0, i16::MAX, Path::new("a440_intervals.wav"))
}
